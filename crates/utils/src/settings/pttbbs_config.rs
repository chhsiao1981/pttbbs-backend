use doku::Document;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
pub struct PTTBBSConfig {
    /// PTTBBS specific configurations.

    /// URL-prefix for go-pttbbs
    #[default("http://localhost:3456/v1")]
    pub prefix: String,

    /// filename for big_to_utf8.
    #[default("docs/examples/etc/pttbbs-backend/uao250-b2u.big5.txt")]
    pub big_to_utf8: String,

    /// filename for big_to_utf8.
    #[default("docs/examples/etc/pttbbs-backend/uao250-u2b.big5.txt")]
    pub utf8_to_big5: String,

    /// filename for ambcjk.
    #[default("docs/examples/etc/pttbbs-backend/ambcjk.big5.txt")]
    pub ambcjk: String,

    /// guest-only mode
    #[default(false)]
    pub is_guest_only: bool,

    /// to retrieve hotboard from pttweb.
    #[default("http://localhost:3457/static/ptt_cc_websites/bbs/HotBoards.html")]
    pub pttweb_hotboard_url: String,

    /// to retrieve other urls from pttweb.
    #[default("http://localhost:3457/static/ptt_cc_websites")]
    pub pttweb_base_url: String,

    /// filename for brdname-whitelist (always retrieving pttweb brdname info).
    #[default("docs/examples/etc/pttbbs-backend/brdname_white_list.yaml")]
    pub brdname_whitelist_map_filename: String,

    /// filename for brdname-whitelist (never retrieving pttweb brdname info).
    #[default("docs/examples/etc/pttbbs-backend/brdname_black_list.yaml")]
    pub brdname_blacklist_map_filename: String,
}
