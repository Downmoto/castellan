use dioxus::prelude::*;

#[component]
pub fn BlockComponent(children: Element, flex_amount: u16) -> Element {
    rsx! {
        div { class: format!("min-w-0 flex-{flex_amount} overflow-hidden rounded-3xl bg-neutral-100 p-4"),
            {children}
        }
    }
}
