use serde::{Deserialize, Deserializer};
use tracing::level_filters::LevelFilter;

#[derive(Debug, Deserialize)]
pub struct AppLogSettings {
    #[serde(default = "default_level_filter", deserialize_with = "deserialize_level_filter")]
    pub level: LevelFilter,
}


impl Default for AppLogSettings {
    fn default() -> Self {
        Self {
            level: default_level_filter(),
        }
    }
}

fn default_level_filter() -> LevelFilter {
    LevelFilter::DEBUG
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