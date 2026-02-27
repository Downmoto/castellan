use tracing::Subscriber;
use tracing_subscriber::Layer;

/// placeholder tracing layer for application file logging.
pub struct AppFileLayer;

impl AppFileLayer {
    /// creates the file logging layer.
    pub fn new() -> Self {
        Self {}
    }
}

impl<S: Subscriber> Layer<S> for AppFileLayer {}
