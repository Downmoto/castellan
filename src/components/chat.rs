use dioxus::prelude::*;

#[component]
pub fn Chat(messages: Signal<Vec<String>>) -> Element {
    let current_messages = messages();

    rsx! {
        div { class: "h-full w-full overflow-y-auto rounded-lg border border-secondary-200 bg-neutral-50 p-3",
            if current_messages.is_empty() {
                div { class: "grid h-full place-items-center rounded-md border border-dashed border-secondary-200",
                    p { class: "text-sm text-secondary-500", "start the conversation" }
                }
            }

            for (idx , message) in current_messages.iter().enumerate() {
                div { key: "{idx}", class: "mb-2.5 flex",
                    p { class: "max-w-[85%] rounded-md bg-secondary-700 px-3 py-2 text-sm leading-relaxed text-primary-50",
                        "{message}"
                    }
                }
            }
        }
    }
}
