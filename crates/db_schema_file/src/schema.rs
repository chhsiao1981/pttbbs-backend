// @generated automatically by Diesel CLI.

pub use lemmy_db_schema_file::schema::{comment, post};

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "comment_type_enum"))]
    pub struct CommentTypeEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CommentTypeEnum;

    ptt_comment (id) {
        id -> Uuid,
        comment_id -> Int4,
        the_type -> Nullable<CommentTypeEnum>,
        ip -> Nullable<Text>,
        host -> Nullable<Text>,
        the_date -> Nullable<Text>,
        dbcs -> Nullable<Bytea>,
        edit_time -> Nullable<Timestamptz>,
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

diesel::joinable!(ptt_comment -> comment (comment_id));
diesel::joinable!(ptt_post -> post (post_id));
diesel::joinable!(ptt_post_comment -> post (post_id));

diesel::allow_tables_to_appear_in_same_query!(ptt_comment, ptt_post, ptt_post_comment);
