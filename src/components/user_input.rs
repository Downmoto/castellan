use dioxus::prelude::*;

#[component]
pub fn UserInput(on_submit: EventHandler<String>) -> Element {
    let mut draft = use_signal(String::new);

    rsx! {
        form {
            class: "mt-3 flex w-full gap-2",
            onsubmit: move |evt| {
                evt.prevent_default();
                let text = draft().trim().to_string();
                if text.is_empty() {
                    return;
                }
                on_submit.call(text);
                draft.set(String::new());
            },

            input {
                class: "flex-1 rounded-md border border-neutral-700 bg-neutral-900 px-3 py-2 text-sm text-neutral-100 outline-none",
                r#type: "text",
                value: draft(),
                placeholder: "type a message",
                oninput: move |evt| draft.set(evt.value()),
            }

            button {
                class: "rounded-md border border-neutral-600 bg-neutral-800 px-3 py-2 text-sm text-neutral-100",
                r#type: "submit",
                "send"
            }
        }
    }
}
