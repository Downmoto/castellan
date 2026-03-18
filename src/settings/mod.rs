pub mod settings_logging;
pub mod settings_keybinds;
pub mod settings_scroll;

pub mod prelude {
    use crate::settings::settings_logging::AppLogSettings;
    use crate::settings::settings_keybinds::AppKeybindsSettings;
    use crate::settings::settings_scroll::AppScrollSettings;

    use config::Config;
    use serde::Deserialize;
    use std::sync::OnceLock;
    use thiserror::Error;

    struct SettingsState {
        settings: CastellanSettings,
        used_default_settings: bool,
    }

    fn settings_state() -> &'static SettingsState {
        static SETTINGS_STATE: OnceLock<SettingsState> = OnceLock::new();

        SETTINGS_STATE.get_or_init(|| {
            let settings = CastellanSettings::new();

            match settings {
                Ok(settings) => SettingsState {
                    settings,
                    used_default_settings: false,
                },
                Err(_) => SettingsState {
                    settings: CastellanSettings::default(),
                    used_default_settings: true,
                },
            }
        })
    }

    pub fn settings() -> &'static CastellanSettings {
        &settings_state().settings
    }

    pub fn used_default_settings() -> bool {
        settings_state().used_default_settings
    }

    #[derive(Debug, Default, Deserialize)]
    pub struct CastellanSettings {
        #[serde(default)]
        pub app_log: AppLogSettings,
        #[serde(default)]
        pub keybinds: AppKeybindsSettings,
        #[serde(default)]
        pub scroll: AppScrollSettings,
    }

    impl CastellanSettings {
        fn new() -> Result<Self, SettingError> {
            let config_result: Result<Config, config::ConfigError> = Config::builder()
                .add_source(config::File::with_name("default").required(true))
                .add_source(
                    config::Environment::with_prefix("CAST")
                        .prefix_separator("_")
                        .separator("__"),
                )
                .build();

            let config = config_result?.try_deserialize()?;

            Ok(config)
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
