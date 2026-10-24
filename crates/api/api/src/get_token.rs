use actix_web::{
    HttpRequest,
    web::{Data, Json},
};
use lemmy_api_utils::{claims::Claims, context::LemmyContext, utils::is_admin};
use lemmy_db_views_local_user::LocalUserView;
use lemmy_db_views_site::api::LoginResponse;
use lemmy_diesel_utils::sensitive::SensitiveString;
use lemmy_utils::error::LemmyResult;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// Creates a site. Should be done after first running lemmy.
pub struct GetToken {
    username: SensitiveString,
}

pub async fn get_token(
    Json(data): Json<GetToken>,
    req: HttpRequest,
    context: Data<LemmyContext>,
    local_user_view: LocalUserView,
) -> LemmyResult<Json<LoginResponse>> {
    is_admin(&local_user_view)?;

    let user = LocalUserView::find_by_email_or_name(&mut context.pool(), &data.username).await?;

    let jwt = Claims::generate(user.local_user.id, Some(true), req, &context).await?;

    Ok(Json(LoginResponse {
        jwt: Some(jwt),
        verify_email_sent: false,
        registration_created: false,
    }))
}
