// app wide imports
use anyhow::Result;


use castellan::logging::subscriber::logging_init;

use tracing::{span, info, Level};


#[tokio::main]
async fn main() -> Result<()> {
    let _subscriber = logging_init();

    let _guard = span!(Level::INFO, "castellan_global").entered();
    info!("App start");

    Ok(())
}
