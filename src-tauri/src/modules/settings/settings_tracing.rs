//! logging settings model and deserializers.

use crate::modules::tracing::tracing_console_layer::TimestampMode;

use serde::{Deserialize, Deserializer};
use tracing::level_filters::LevelFilter;

/// logging configuration used during subscriber initialization.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct TracingSettings {
    /// minimum level emitted for application targets.
    #[serde(deserialize_with = "deserialize_level_filter")]
    pub level: LevelFilter,
    /// timestamp format for console events.
    pub timestamp_mode: TimestampMode,
}

/// provides stable defaults when settings are missing or invalid.
impl Default for TracingSettings {
    fn default() -> Self {
        Self {
            level: LevelFilter::DEBUG,
            timestamp_mode: TimestampMode::Utc,
        }
    }
}

fn deserialize_level_filter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    match value.to_ascii_uppercase().as_str() {
        "TRACE" => Ok(LevelFilter::TRACE),
        "DEBUG" => Ok(LevelFilter::DEBUG),
        "INFO" => Ok(LevelFilter::INFO),
        "WARN" => Ok(LevelFilter::WARN),
        "ERROR" => Ok(LevelFilter::ERROR),
        _ => Err(serde::de::Error::custom(format!(
            "invalid log level '{value}', expected one of: TRACE, DEBUG, INFO, WARN, ERROR"
        ))),
    }
}
