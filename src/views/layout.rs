use crate::views::{Route, SidebarView};
use dioxus::prelude::*;

#[component]
pub fn LayoutView() -> Element {
    rsx! {
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

        div { class: "h-full w-full bg-neutral-800 p-4",
            div { class: "flex h-full w-full gap-4",
                SidebarView {}

                main { class: "min-w-0 flex-1 overflow-hidden rounded-3xl border-2 border-neutral-950 bg-neutral-100 p-4",
                    Outlet::<Route> {}
                }
            }
        }
    }
}