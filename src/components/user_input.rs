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
                class: "flex-1 rounded-md border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 outline-none placeholder:text-slate-400 focus:border-slate-500",
                r#type: "text",
                value: draft(),
                placeholder: "type a message",
                oninput: move |evt| draft.set(evt.value()),
            }

            button {
                class: "rounded-md border border-slate-300 bg-slate-100 px-4 py-2 text-sm font-medium text-slate-800 hover:bg-slate-200",
                r#type: "submit",
                "send"
            }
        }
    }
}
