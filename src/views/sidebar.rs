use dioxus::prelude::*;

#[component]
pub fn SidebarView() -> Element {
    rsx! {
        div { class: "flex flex-col justify-center gap-2",
            div { "1" }
            div { "2" }
            div { "3" }
        }
    }
}