pub mod app_console_layer;
pub mod app_file_layer;

pub mod prelude {
    use tracing_subscriber::{layer::SubscriberExt, registry::Registry};

    use crate::logging::app_console_layer::AppConsoleLayer;
    use crate::logging::app_file_layer::AppFileLayer;

    #[derive(Clone)]
    pub enum SubscriberErr {
        InitializationError,
    }

    pub fn logging_init() -> Result<(), SubscriberErr> {
        let sub = Registry::default()
            .with(AppConsoleLayer::new())
            .with(AppFileLayer::new());

        match tracing::subscriber::set_global_default(sub) {
            Err(error) => {
                eprintln!("Failed to set global subscriber: {}", error);
                Err(SubscriberErr::InitializationError)
            }
            Ok(_) => Ok(()),
        }
    }
}
