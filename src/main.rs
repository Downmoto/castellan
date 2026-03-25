use dioxus::prelude::*;

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

fn main() {
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

        div { class: "min-h-screen grid place-items-center",
            h1 { class: "text-3xl font-bold", "hello from castellan" }
        }

    }
}
