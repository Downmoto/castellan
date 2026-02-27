use castellan::logging::prelude::*;
use castellan::settings::prelude::*;

use tracing::{Level, event, span};

#[tokio::main]
async fn main() {
    let settings = settings();
    let _subscriber = logging_init(settings.app_log().level, settings.app_log().timestamp_mode);

    let _guard = span!(Level::INFO, "castellan_global").entered();
    event!(Level::INFO, "App start");

    if used_default_settings() {
        event!(Level::WARN, "Failed to parse configuration; using defaults")
    }
}
