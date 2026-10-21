use chrono::{DateTime, Utc};
#[cfg(feature = "full")]
use i_love_jesus::CursorKeysModule;
#[cfg(feature = "full")]
use lemmy_db_schema::source::post::Post;
use pttbbs_db_schema_file::enums::CommentType;
use pttbbs_db_schema_file::schema::ptt_post;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "full",
    derive(Queryable, Selectable, Associations, Identifiable, CursorKeysModule)
)]
#[cfg_attr(feature = "full", diesel(table_name = ptt_post))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "full", cursor_keys_module(name = post_keys))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// PTT-specific info for Posts.
pub struct PttCommunity {
    pub id: uuid::Uuid,
    pub community_id: i32,
    pub ptt_brdname: Option<String>,

    pub bclass: Option<String>,

    pub brdattr: i32,
    pub vote_limit_logins: i32,
    pub post_limit_logins: i32,
    pub bvote: i32,
    pub vote_end_time: Option<DateTime<Utc>>,
    pub perm: i32,
    pub perm_update_time: Option<DateTime<Utc>>,
    pub gamble_end_time: Option<DateTime<Utc>>,
    pub fastrecommend_pause: Option<i32>,
    pub vote_limit_badpost: i32,
    pub post_limit_badpost: i32,
    pub bottom: Vec<i32>,
}
