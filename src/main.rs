use castellan::{
    settings::{settings, used_default_settings},
    tracing::logging_init,
    views::Route,
};

use dioxus::prelude::*;
use tracing::{event, span, Level};

#[cfg(feature = "desktop")]
use dioxus::desktop::{
    tao::{dpi::LogicalSize, window::WindowBuilder},
    Config,
};

fn main() {
    let settings = settings();
    let _subscriber = logging_init(settings.tracing.level, settings.tracing.timestamp_mode);

    let _guard = span!(Level::INFO, "castellan_global").entered();
    event!(Level::INFO, "App start");

    if used_default_settings() {
        event!(Level::WARN, "Failed to parse configuration; using defaults");
    }

    launch();
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        Router::<Route> {}
    }
}

fn launch() {
    #[cfg(feature = "desktop")]
    {
        dioxus::LaunchBuilder::desktop()
            .with_cfg(
                Config::new().with_window(
                    WindowBuilder::new()
                        .with_inner_size(LogicalSize::new(1280.0, 840.0))
                        .with_min_inner_size(LogicalSize::new(960.0, 640.0))
                        .with_title(String::from("castellan")),
                ),
            )
            .launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    {
        dioxus::launch(App);
    }
}
