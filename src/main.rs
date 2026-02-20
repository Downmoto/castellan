use tracing::{Level, event, span};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let span = span!(Level::TRACE, "app_start");
    let _enter = span.enter();

    event!(Level::INFO, "Application has started");
}
