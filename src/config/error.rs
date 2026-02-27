use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingError {
    #[error("Could not parse setting from file or env vars")]
    DeserializeError
}

impl From<config::ConfigError> for SettingError {
    fn from(_: config::ConfigError) -> Self {
        SettingError::DeserializeError
    }
}