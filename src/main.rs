use castellan::{logging::prelude::*};

use tracing::{span, info, Level};


#[tokio::main]
async fn main() {
    let _subscriber = logging_init();

    let _guard = span!(Level::INFO, "castellan_global").entered();
    info!("App start");

    let s = &castellan::config::settings().app_log_settings().level;
    print!("{s:?}");
}
