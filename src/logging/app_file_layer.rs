use tracing::Subscriber;
use tracing_subscriber::Layer;

pub struct AppFileLayer;

impl AppFileLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S: Subscriber> Layer<S> for AppFileLayer {}
