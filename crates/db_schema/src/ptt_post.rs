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
pub struct PttPost {
    pub id: uuid::Uuid,
    pub post_id: i32,
    pub ptt_id: Option<String>,

    /// classes defined by moderators.
    pub class: Option<String>,

    pub content: Option<String>,

    pub money: i32,
    pub ip: Option<String>,
    pub host: Option<String>,
    pub bbs: Option<String>,
    pub is_highlight: bool,
    pub full_title: Option<String>,
    pub full_title_big5: Option<Vec<u8>>,
    pub content_big5: Option<Vec<u8>>,
    pub signature_big5: Option<Vec<u8>>,
    pub content_big5_md5: Option<String>,
    pub signature_big5_md5: Option<String>,
}
