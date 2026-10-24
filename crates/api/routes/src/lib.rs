use actix_web::web::{ServiceConfig, post, scope};
use lemmy_utils::rate_limit::RateLimit;
use pttbbs_api::get_token;

pub fn config(cfg: &mut ServiceConfig, rate_limit: &RateLimit) {
    cfg.service(
        scope("/api/v4")
            .wrap(rate_limit.message())
            .service(scope("/pttbbs-lemmy").route("/token", post().to(get_token))),
    );
}
