use dioxus::prelude::*;

#[component]
pub fn UserInput(on_submit: EventHandler<String>) -> Element {
    let mut draft = use_signal(String::new);

    rsx! {
        form {
            class: "mt-3 flex w-full items-center gap-2",
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
                class: "flex-1 rounded-md border border-secondary-300 bg-neutral-50 px-3 py-2 text-sm text-secondary-900 outline-none placeholder:text-secondary-400 focus:border-primary-500",
                r#type: "text",
                value: draft(),
                placeholder: "type a message",
                oninput: move |evt| draft.set(evt.value()),
            }

            button {
                class: "rounded-md border border-secondary-300 bg-tertiary-300 px-4 py-2 text-sm font-medium text-secondary-900 hover:bg-tertiary-400",
                r#type: "submit",
                "send"
            }
        }
    }
}
