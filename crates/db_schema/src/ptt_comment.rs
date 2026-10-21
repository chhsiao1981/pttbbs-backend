use chrono::{DateTime, Utc};
use lemmy_db_schema::source::comment::Comment;
use pttbbs_db_schema_file::enums::CommentType;
use serde::{Deserialize, Serialize};

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "full",
    derive(Queryable, Selectable, Associations, Identifiable, CursorKeysModule)
)]
#[cfg_attr(feature = "full", diesel(belongs_to(crate::ptt_post::PttPost)))]
#[cfg_attr(feature = "full", diesel(table_name = ptt_comment))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "full", cursor_keys_module(name = comment_keys))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
pub struct PttComment {
    id: uuid::Uuid,
    comment_id: i32,
    ptt_id: Option<String>,
    the_type: Option<CommentType>,
    ip: Option<String>,
    host: Option<String>,
    the_date: Option<String>,
    dbcs: Option<Vec<u8>>,
    edit_time: Option<DateTime<Utc>>,
    content_md5: Option<String>,
}
