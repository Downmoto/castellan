use tracing::Subscriber;
use tracing_subscriber::{
    Layer,
    layer::{Layered, SubscriberExt},
    registry::Registry,
};

use crate::logging::app_console_layer::AppConsoleLayer;

pub enum SubscriberErr {
    Default,
}

pub struct AppFileLayer;

impl AppFileLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S: Subscriber> Layer<S> for AppFileLayer {}

type MultiLayeredSubscriber =
    Layered<AppFileLayer, Layered<AppConsoleLayer, Registry>>;

pub fn logging_init() -> Result<(), SubscriberErr> {
    let sub: MultiLayeredSubscriber = Registry::default()
        .with(AppConsoleLayer::new())
        .with(AppFileLayer::new());

    match tracing::subscriber::set_global_default(sub) {
        Err(error) => {
            eprintln!("Failed to set global subscriber: {}", error);
            Err(SubscriberErr::Default)
        }
        Ok(_) => Ok(()),
    }
}
