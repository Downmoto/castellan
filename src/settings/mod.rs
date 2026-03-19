//! settings loading and typed configuration access.
//!
//! configuration sources are loaded once and cached for process lifetime.
//! values come from `default.toml` and optional `CAST_*` environment overrides.

/// logging-related settings models.
pub mod settings_logging;
/// keybind settings models and deserializers.
pub mod settings_keybinds;
/// transcript and viewport scroll settings models.
pub mod settings_scroll;

/// one-time settings accessors and shared settings types.
pub mod prelude {
    use std::sync::OnceLock;

    use crate::settings::settings_logging::AppLogSettings;
    use crate::settings::settings_keybinds::AppKeybindsSettings;
    use crate::settings::settings_scroll::AppScrollSettings;

    use config::Config;
    use serde::Deserialize;
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

    /// returns process-global settings initialized on first access.
    ///
    /// if parsing fails, this returns default settings and sets
    /// [`used_default_settings`] to `true`.
    pub fn settings() -> &'static CastellanSettings {
        &settings_state().settings
    }

    /// reports whether startup fell back to default settings.
    pub fn used_default_settings() -> bool {
        settings_state().used_default_settings
    }

    /// top-level settings object used by the application.
    #[derive(Debug, Default, Deserialize)]
    pub struct CastellanSettings {
        /// logging filter and timestamp rendering options.
        #[serde(default)]
        pub app_log: AppLogSettings,
        /// command bindings grouped by interaction mode.
        #[serde(default)]
        pub keybinds: AppKeybindsSettings,
        /// line and page movement amounts for transcript scrolling.
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

    /// setting parse/load failure for file and env sources.
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
