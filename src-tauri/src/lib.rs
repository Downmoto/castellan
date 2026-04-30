pub mod modules;

use crate::modules::settings::{settings, used_default_settings};
use crate::modules::tracing::logging_init;

use dotenv::dotenv;

use tracing::{span, event, Level};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

    let settings = settings();
    let _subscriber = logging_init(settings.tracing.level, settings.tracing.timestamp_mode);

    let _guard = span!(Level::INFO, "castellan_global").entered();
    event!(Level::INFO, "App start");

    if used_default_settings() {
        event!(Level::WARN, "Failed to parse configuration; using defaults");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
