use actix_web::{
    App, HttpResponse, HttpServer,
    dev::{ServerHandle, ServiceResponse},
    middleware::{self, Condition, ErrorHandlerResponse, ErrorHandlers},
    web::{Data, get, scope},
};
use lemmy_api_utils::{
    context::LemmyContext, request::client_builder,
    utils::local_site_rate_limit_to_rate_limit_config,
};
use lemmy_db_schema::source::secret::Secret;
use lemmy_diesel_utils::connection::build_db_pool;
use lemmy_routes::utils::{
    prometheus_metrics::serve_prometheus, setup_local_site::setup_local_site,
};
use lemmy_server::CmdArgs;
use lemmy_utils::{VERSION, error::LemmyResult, rate_limit::RateLimit, settings::SETTINGS};
use reqwest_middleware::ClientBuilder;
use reqwest_tracing::TracingMiddleware;
use tokio::signal::unix::SignalKind;

pub async fn start_server(args: CmdArgs) -> LemmyResult<()> {
    // Print version number to log
    println!("Starting Lemmy v{}", *VERSION);

    // return error 503 while running db migrations and startup tasks
    let mut startup_server_handle = create_startup_server()?;

    // Set up the connection pool
    let pool = build_db_pool()?;

    // Initialize the secrets
    let secret = Secret::init(&mut (&pool).into()).await?;

    // Make sure the local site is set up.
    let site_view = setup_local_site(&mut (&pool).into(), &SETTINGS).await?;

    // Set up the rate limiter
    let rate_limit_config =
        local_site_rate_limit_to_rate_limit_config(&site_view.local_site_rate_limit);
    let rate_limit_cell = RateLimit::new(rate_limit_config);

    println!(
        "Starting HTTP server at {}:{}",
        SETTINGS.bind, SETTINGS.port
    );

    let client = ClientBuilder::new(client_builder(&SETTINGS).build()?)
        .with(TracingMiddleware::default())
        .build();
    let pictrs_client = ClientBuilder::new(client_builder(&SETTINGS).no_proxy().build()?)
        .with(TracingMiddleware::default())
        .build();
    let context = LemmyContext::create(
        pool.clone(),
        client.clone(),
        pictrs_client,
        secret.clone(),
        rate_limit_cell,
    );

    if let Some(prometheus) = SETTINGS.prometheus.clone() {
        serve_prometheus(prometheus, context.clone())?;
    }

    startup_server_handle.stop(true).await?;
    let server = create_http_server(SETTINGS.clone(), site_view)?;

    // This FederationConfig instance is exclusively used to send activities, so we can safely
    // increase the timeout without affecting timeouts for resolving objects anywhere.
    let mut interrupt = tokio::signal::unix::signal(SignalKind::interrupt())?;
    let mut terminate = tokio::signal::unix::signal(SignalKind::terminate())?;

    tokio::select! {
      _ = tokio::signal::ctrl_c() => {
        tracing::warn!("Received ctrl-c, shutting down gracefully...");
      }
      _ = interrupt.recv() => {
        tracing::warn!("Received interrupt, shutting down gracefully...");
      }
      _ = terminate.recv() => {
        tracing::warn!("Received terminate, shutting down gracefully...");
      }
    }
    if let Some(server) = server {
        server.stop(true).await;
    }

    Ok(())
}

fn create_startup_server() -> LemmyResult<ServerHandle> {
    let startup_server = HttpServer::new(move || {
        App::new().wrap(ErrorHandlers::new().default_handler(move |req| {
            let (req, _) = req.into_parts();
            let response = HttpResponse::ServiceUnavailable()
                .json(json!({"error": "Lemmy is currently starting"}));
            let service_response = ServiceResponse::new(req, response);
            Ok(ErrorHandlerResponse::Response(
                service_response.map_into_right_body(),
            ))
        }))
    })
    .bind((SETTINGS.bind, SETTINGS.port))?
    .run();
    let startup_server_handle = startup_server.handle();
    tokio::task::spawn(startup_server);
    Ok(startup_server_handle)
}

fn create_http_server(settings: Settings, site_view: SiteView) -> LemmyResult<ServerHandle> {
    // These must come before HttpServer creation so they can collect data across threads.
    let prom_api_metrics = new_prometheus_metrics()?;
    let idempotency_set = IdempotencySet::default();

    // Create Http server
    let bind = (settings.bind, settings.port);
    let server = HttpServer::new(move || {
        let context: LemmyContext = federation_config.deref().clone();
        let rate_limit = federation_config.rate_limit_cell().clone();

        let cors_config = cors_config(&settings);
        let app = App::new()
            .wrap(ErrorHandlers::new().default_handler(jsonify_plain_text_errors))
            .wrap(middleware::Logger::new(
                // This is the default log format save for the usage of %{r}a over %a to guarantee to
                // record the client's (forwarded) IP and not the last peer address, since the latter is
                // frequently just a reverse proxy
                "%{r}a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %T",
            ))
            .wrap(middleware::Compress::default())
            .wrap(cors_config)
            .wrap(TracingLogger::<DefaultRootSpanBuilder>::new())
            .app_data(Data::new(context.clone()))
            .wrap(IdempotencyMiddleware::new(idempotency_set.clone()))
            .wrap(SessionMiddleware::new(context.clone()))
            .wrap(Condition::new(
                SETTINGS.prometheus.is_some(),
                prom_api_metrics.clone(),
            ));

        // The routes
        app.configure(|cfg| lemmy_api_routes::config(cfg, &rate_limit))
            .configure(|cfg| lemmy_api_routes_v3::config(cfg, &rate_limit))
            .configure(|cfg| {
                if site_view.local_site.federation_enabled {
                    lemmy_apub::http::routes::config(cfg);
                    webfinger::config(cfg);
                }
            })
            .configure(|cfg| feeds::config(cfg, &rate_limit))
            .configure(nodeinfo::config)
            .service(
                scope("/sitemap.xml")
                    .wrap(rate_limit.message())
                    .route("", get().to(get_sitemap)),
            )
    })
    .disable_signals()
    .bind(bind)?
    .run();
    let handle = server.handle();
    tokio::task::spawn(server);
    Ok(handle)
}
