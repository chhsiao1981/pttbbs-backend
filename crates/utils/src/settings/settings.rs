use super::pttbbs_config::PTTBBSConfig;
use deser_hjson::from_str;
use doku::Document;
use lemmy_utils::error::LemmyResult;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::{env, fs};

/// XXX should be same as lemmy_utils::settings::DEFAULT_CONFIG_FILE.
static DEFAULT_CONFIG_FILE: &str = "config/config.hjson";

#[derive(Debug, Deserialize, Serialize, Clone, SmartDefault, Document)]
#[serde(default)]
pub struct Settings {
    /// pttbbs configuration.
    pub pttbbs: PTTBBSConfig,
}

impl Settings {
    pub(crate) fn init() -> LemmyResult<Self> {
        let path =
            env::var("LEMMY_CONFIG_LOCATION").unwrap_or_else(|_| DEFAULT_CONFIG_FILE.to_string());
        let plain = fs::read_to_string(path)?;
        let config = from_str::<Settings>(&plain)?;
        Ok(config)
    }
}
