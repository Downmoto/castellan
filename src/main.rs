use castellan::logging::prelude::*;

use tracing::{Level, info, span};


#[tokio::main]
async fn main() {
    let settings = castellan::settings::settings();
    let _subscriber = logging_init(settings.app_log().level);

    let _guard = span!(Level::INFO, "castellan_global").entered();
    info!("App start");
}
