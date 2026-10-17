use chrono::{DateTime, Utc};
#[cfg(feature = "full")]
use i_love_jesus::CursorKeysModule;
use lemmy_db_schema::source::post::Post;
use pttbbs_db_schema_file::enums::CommentType;
#[cfg(feature = "full")]
use pttbbs_db_schema_file::schema::ptt_post_comment;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "full",
    derive(Queryable, Selectable, Associations, Identifiable, CursorKeysModule)
)]
#[cfg_attr(feature = "full", diesel(table_name = ptt_pos_comment))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "full", cursor_keys_module(name = post_comment_keys))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// Comment-related info for posts.
pub struct PttPostComment {
    pub id: uuid::Uuid,
    pub recommend: i32,
    pub n_comment: i32,
    pub post_id: i32,
}
