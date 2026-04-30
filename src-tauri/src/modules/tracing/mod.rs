//! logging setup and layers used by the application.

/// tracing layer implementation for terminal console output.
pub mod tracing_console_layer;

/// convenient exports and initialization helpers for application logging.
use crate::modules::tracing::tracing_console_layer::{TracingConsoleLayer, TimestampMode};

use thiserror::Error;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::Layer;
use tracing_subscriber::{layer::SubscriberExt, registry::Registry};

/// error returned when subscriber initialization fails.
#[derive(Clone, Debug, Error)]
pub enum SubscriberErr {
    #[error("Failed to set global subscriber, {0}")]
    InitializationError(String),
}

/// initializes the global tracing subscriber for the process.
///
/// this wires the console and file layers and applies the provided level filter
/// to the console output.
pub fn logging_init(
    app_log_filter: LevelFilter,
    timestamp_mode: TimestampMode,
) -> Result<(), SubscriberErr> {
    let app_targets = Targets::new().with_target("castellan", app_log_filter);

    let sub =
        Registry::default().with(TracingConsoleLayer::new(timestamp_mode).with_filter(app_targets));

    if let Err(e) = tracing::subscriber::set_global_default(sub) {
        return Err(SubscriberErr::InitializationError(e.to_string()));
    };

    Ok(())
}
