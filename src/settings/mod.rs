pub mod settings_logging;

pub mod prelude {
    use crate::settings::settings_logging::AppLogSettings;

    use config::Config;
    use serde::Deserialize;
    use std::sync::OnceLock;
    use thiserror::Error;

    pub fn settings() -> &'static CastellanSettings {
        static SETTINGS: OnceLock<CastellanSettings> = OnceLock::new();

        SETTINGS.get_or_init(|| {
            let settings = CastellanSettings::new();

            match settings {
                Ok(settings) => settings,
                Err(err) => panic!("{err}"),
            }
        })
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct CastellanSettings {
        app_log: AppLogSettings,
    }

    impl CastellanSettings {
        fn new() -> Result<Self, SettingError> {
            let config_result: Result<Config, config::ConfigError> = Config::builder()
                .add_source(config::File::with_name("default").required(false))
                .add_source(
                    config::Environment::with_prefix("CAST")
                        .prefix_separator("_")
                        .separator("__"),
                )
                .build();

            let config = match config_result {
                Ok(c) => c,
                Err(_) => return Ok(Self::default()),
            }
            .try_deserialize::<CastellanSettings>()?;

            Ok(config)
        }

        pub fn app_log(&self) -> &AppLogSettings {
            &self.app_log
        }
    }

    #[derive(Error, Debug)]
    pub enum SettingError {
        #[error("Could not parse setting from file or env vars")]
        DeserializeError,
    }

    impl From<config::ConfigError> for SettingError {
        fn from(_: config::ConfigError) -> Self {
            SettingError::DeserializeError
        }
    }
}
