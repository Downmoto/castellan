use crate::views::Route;
use dioxus::prelude::*;

#[component]
pub fn SidebarView() -> Element {
    rsx!(
        div { "hello" }
        Outlet::<Route> {}
    )
}