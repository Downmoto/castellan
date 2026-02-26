use std::sync::OnceLock;

use config::Config;
use serde::Deserialize;
use tracing::{Level, info, span, warn};

pub enum SettingError {
    DeserializeError
}


pub fn settings() -> &'static CastellanSettings {
    static SETTINGS:  OnceLock<CastellanSettings> = OnceLock::new();

    SETTINGS.get_or_init(|| {
        let settings = CastellanSettings::new();

        match settings {
            Ok(settings) => settings,
            Err(err) => panic!("settings double fail {err}")
        }
    })

}


#[derive(Debug, Deserialize)]
pub enum AppLogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR
}

#[derive(Debug, Deserialize)]
pub struct AppLogSettings {
    pub level: AppLogLevel
}

#[derive(Debug, Deserialize)]
pub struct CastellanSettings {
    app_log: AppLogSettings
}

impl CastellanSettings {
    fn new() -> Result<Self, config::ConfigError> {
        let _guard = span!(Level::WARN, "Config").entered();
        info!("Initializing Castellan Configuration");

        let config_result = 
            Config::builder()
                .add_source(config::File::with_name("default"))
                .add_source(
                    config::Environment::with_prefix("CAST")
                        .prefix_separator("_")
                        .separator("__")
                )
                .build();

        let config = match config_result {
            Ok(c) => {
                info!("Successfully loaded configs");
                c
            },
            Err(_) => {
                warn!("Failed to load configs");
                Config::default()
            }
        }.try_deserialize();

        config
    }

    pub fn app_log_settings(&self) -> &AppLogSettings {
        &self.app_log
    }
}