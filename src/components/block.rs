use dioxus::prelude::*;

#[component]
pub fn BlockComponent(children: Element, flex_amount: u16) -> Element {
    rsx! {
        div {
            class: "overflow-hidden rounded-3xl",
            style: format!("flex: {} 1 0%;", flex_amount),
            {children}
        }
    }
}
