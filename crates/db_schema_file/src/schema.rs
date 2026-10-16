// @generated automatically by Diesel CLI.

use lemmy_db_schema_file::schema::{comment, post};

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "comment_type"))]
    pub struct CommentType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CommentType;

    ptt_comment (id) {
        id -> Uuid,
        comment_id -> Int4,
        the_type -> Nullable<CommentType>,
        ip -> Nullable<Text>,
        host -> Nullable<Text>,
        the_date -> Nullable<Text>,
        dbcs -> Nullable<Text>,
        edit_time_us -> Nullable<Timestamptz>,
        content_md5 -> Nullable<Text>,
    }
}

diesel::table! {
    ptt_post (id) {
        id -> Uuid,
        post_id -> Int4,
        class -> Nullable<Text>,
        content -> Nullable<Text>,
        money -> Int4,
        ip -> Nullable<Text>,
        host -> Nullable<Text>,
        bbs -> Nullable<Text>,
        is_highlight -> Bool,
        full_title -> Nullable<Text>,
        full_title_big5 -> Nullable<Bytea>,
        content_big5 -> Nullable<Bytea>,
        signature_big5 -> Nullable<Bytea>,
        content_big5_md5 -> Nullable<Text>,
        signature_big5_md5 -> Nullable<Text>,
    }
}

diesel::table! {
    ptt_post_comment (id) {
        id -> Uuid,
        recommend -> Int4,
        n_comment -> Int4,
        post_id -> Int4,
    }
}

diesel::joinable!(comment -> post (post_id));
diesel::joinable!(ptt_comment -> comment (comment_id));
diesel::joinable!(ptt_post -> post (post_id));
diesel::joinable!(ptt_post_comment -> post (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    comment,
    post,
    ptt_comment,
    ptt_post,
    ptt_post_comment,
);
