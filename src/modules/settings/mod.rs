//! settings loading and typed configuration access.
//!
//! configuration sources are loaded once and cached for process lifetime.
//! values come from `default.toml` and optional `CAST_*` environment overrides.

/// tracing-related settings models.
pub mod settings_tracing;

/// one-time settings accessors and shared settings types.
use crate::settings::settings_tracing::TracingSettings;

use std::sync::OnceLock;

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
    pub tracing: TracingSettings,
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
