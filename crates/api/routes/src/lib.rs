use actix_web::web::{ServiceConfig, post, scope};
use lemmy_api_crud::site::read::get_site;
use lemmy_utils::rate_limit::RateLimit;

pub fn config(cfg: &mut ServiceConfig, rate_limit: &RateLimit) {
    cfg.service(
        scope("/api/v4")
            .wrap(rate_limit.message())
            .service(scope("/pttbbs-lemmy").route("/token", post().to(get_site))),
    );
}
