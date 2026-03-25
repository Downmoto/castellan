use castellan::{
    settings::{settings, used_default_settings},
    tracing::logging_init,
    views::ChatView,
};

use dioxus::prelude::*;
use tracing::{event, span, Level};

fn main() {
    let settings = settings();
    let _subscriber = logging_init(settings.tracing.level, settings.tracing.timestamp_mode);

    let _guard = span!(Level::INFO, "castellan_global").entered();
    event!(Level::INFO, "App start");

    if used_default_settings() {
        event!(Level::WARN, "Failed to parse configuration; using defaults");
    }

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        style {
            "body {{
                margin: 0; 
                overflow-x: hidden; 
                overflow-y: hidden; 
            }}"
        }

        div { class: "min-h-screen grid place-items-center", ChatView {} }

    }
}
