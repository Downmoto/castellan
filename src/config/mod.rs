pub mod error;
use crate::config::error::SettingError;

use std::sync::OnceLock;

use config::Config;
use serde::Deserialize;

pub fn settings() -> &'static CastellanSettings {
    static SETTINGS:  OnceLock<CastellanSettings> = OnceLock::new();

    SETTINGS.get_or_init(|| {
        let settings = CastellanSettings::new();

        match settings {
            Ok(settings) => settings,
            Err(err) => panic!("{err}")
        }
    })

}


#[derive(Debug, Default, Deserialize)]
pub enum AppLogLevel {
    #[default]
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR
}

#[derive(Debug, Default, Deserialize)]
pub struct AppLogSettings {
    pub level: AppLogLevel
}

#[derive(Debug, Default, Deserialize)]
pub struct CastellanSettings {
    app_log: AppLogSettings
}

impl CastellanSettings {
    fn new() -> Result<Self, SettingError> {
        let config_result: Result<Config, config::ConfigError> = 
            Config::builder()
                .add_source(config::File::with_name("default")) // return to fix with env::home_dir app init 
                .add_source(
                    config::Environment::with_prefix("CAST")
                        .prefix_separator("_")
                        .separator("__")
                )
                .build();

        let config = match config_result {
            Ok(c) => c,
            Err(_) => return Ok(Self::default())
        }.try_deserialize::<CastellanSettings>()?;

        Ok(config)
    }

    pub fn app_log(&self) -> &AppLogSettings {
        &self.app_log
    }
}