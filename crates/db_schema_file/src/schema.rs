// @generated automatically by Diesel CLI.

pub use lemmy_db_schema_file::schema::{comment, community, post};

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
        ptt_id -> Nullable<Text>,
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
    ptt_community (id) {
        id -> Uuid,
        community_id -> Int4,
        ptt_brdname -> Nullable<Text>,
        bclass -> Nullable<Text>,
        brdattr -> Int4,
        vote_limit_logins -> Int4,
        post_limit_logins -> Int4,
        bvote -> Int4,
        vote_end_time -> Nullable<Timestamptz>,
        perm -> Int4,
        perm_update_time -> Nullable<Timestamptz>,
        gamble_end_time -> Nullable<Timestamptz>,
        fastrecommend_pause -> Nullable<Int4>,
        vote_limit_badpost -> Int4,
        post_limit_badpost -> Int4,
        /// XXX Array<Nullable<Int4>> => Array<Int4>.
        bottom -> Array<Int4>,
    }
}

diesel::table! {
    ptt_post (id) {
        id -> Uuid,
        post_id -> Int4,
        ptt_id -> Nullable<Text>,
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
diesel::joinable!(ptt_community -> community (community_id));
diesel::joinable!(ptt_post -> post (post_id));
diesel::joinable!(ptt_post_comment -> post (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    ptt_comment,
    ptt_community,
    ptt_post,
    ptt_post_comment,
);
