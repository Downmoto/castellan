pub mod app_console_layer;
pub mod app_file_layer;

pub mod prelude {
    use crate::logging::app_console_layer::{AppConsoleLayer, TimestampMode};
    use crate::logging::app_file_layer::AppFileLayer;

    use thiserror::Error;
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::Layer;
    use tracing_subscriber::{layer::SubscriberExt, registry::Registry};

    #[derive(Clone, Debug, Error)]
    pub enum SubscriberErr {
        #[error("Failed to set global subscriber, {0}")]
        InitializationError(String),
    }

    pub fn logging_init(
        app_log_filter: LevelFilter,
        timestamp_mode: TimestampMode,
    ) -> Result<(), SubscriberErr> {
        let sub = Registry::default()
            .with(AppConsoleLayer::new(timestamp_mode).with_filter(app_log_filter))
            .with(AppFileLayer::new());

        if let Err(e) = tracing::subscriber::set_global_default(sub) {
            return Err(SubscriberErr::InitializationError(e.to_string()));
        };

        Ok(())
    }
}
