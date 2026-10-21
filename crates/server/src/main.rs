use lemmy_server::start_lemmy_server;
use lemmy_utils::{error::LemmyResult, settings::SETTINGS};
use pttbbs_crons::retry_load_popular_boards;
use pttbbs_server::parse_args;
use pttbbs_utils::settings::SETTINGS as PTTBBS_SETTINGS;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
pub async fn main() -> LemmyResult<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    if SETTINGS.json_logging {
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .json()
            .init();
    } else {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    }

    let pttbbs_config = &PTTBBS_SETTINGS.pttbbs;

    println!("PTTBBS settings: {PTTBBS_SETTINGS:?} pttbbs_config: {pttbbs_config:?}");

    let args = parse_args();

    let handle = tokio::task::spawn(async move { retry_load_popular_boards().await });
    start_lemmy_server(args.lemmy_args).await?;

    let _ = handle.await?;

    Ok(())
}
