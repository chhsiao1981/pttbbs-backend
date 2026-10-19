mod pttbbs_config;
mod settings;

use self::settings::Settings;
use std::env;
use std::sync::LazyLock;

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| {
    if env::var("LEMMY_INITIALIZE_WITH_DEFAULT_SETTINGS").is_ok() {
        println!(
            "LEMMY_INITIALIZE_WITH_DEFAULT_SETTINGS was set, any configuration file has been ignored."
        );
        println!(
            "Use with other environment variables to configure this instance further; e.g. LEMMY_DATABASE_URL."
        );
        Settings::default()
    } else {
        Settings::init()
            .expect("Failed to load settings file, see documentation (https://docs.devptt.dev).")
    }
});
