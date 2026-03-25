use dioxus::prelude::*;

#[component]
pub fn Chat(messages: Signal<Vec<String>>) -> Element {
    let current_messages = messages();

    rsx! {
        div { class: "w-full rounded-md border border-neutral-700 bg-neutral-900 p-3",
            if current_messages.is_empty() {
                p { class: "text-sm text-neutral-400", "no messages yet" }
            }

            for (idx , message) in current_messages.iter().enumerate() {
                p { key: "{idx}", class: "text-sm text-neutral-100", "{message}" }
            }
        }
    }
}
