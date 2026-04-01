use crate::{components::BlockComponent, views::{Route, SidebarView}};
use dioxus::prelude::*;

#[component]
pub fn LayoutView() -> Element {
    rsx! {
        div { class: "h-full w-full bg-neutral-800 p-4",
            div { class: "flex h-full w-full gap-4",
                SidebarView {}

                BlockComponent { flex_amount: 1 }
                BlockComponent { flex_amount: 3, Outlet::<Route> {} }
            }
        }

        style {
            "html, body {{
                width: 100%;
                height: 100%;
                margin: 0;
                background-color: var(--color-neutral-800);
            }}

            #main {{
                width: 100vw;
                height: 100vh;
                background-color: var(--color-neutral-800);
            }}

            body {{
                overflow-x: hidden;
                overflow-y: hidden;
            }}"
        }
    }
}