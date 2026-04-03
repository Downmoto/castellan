use dioxus::prelude::*;

const INITIAL_VISIBLE_ROWS: usize = 2;
const MAX_VISIBLE_ROWS: usize = 5;

#[component]
pub fn UserInputComponent(on_submit: EventHandler<String>) -> Element {
    let mut draft = use_signal(String::new);
    let textarea_style = format!("min-height: {INITIAL_VISIBLE_ROWS}lh; max-height: {MAX_VISIBLE_ROWS}lh;");

    rsx! {
        form {
            class: "w-full relative group",
            onsubmit: move |evt| {
                evt.prevent_default();
                let text = draft().trim().to_string();
                if text.is_empty() {
                    return;
                }
                on_submit.call(text);
                draft.set(String::new());
            },

            div { class: "relative bg-neutral-800/95 rounded-4xl shadow-lg shadow-black/30 flex flex-col p-6 border border-neutral-700/60",
                textarea {
                    class: "w-full box-border field-sizing-content bg-transparent border-none outline-none focus:border-none focus:outline-none focus:ring-0 focus-visible:outline-none resize-none overflow-y-auto font-body text-lg placeholder:text-neutral-500 text-primary-50 leading-relaxed",
                    style: "{textarea_style}",
                    placeholder: "How can I help you today?",
                    rows: INITIAL_VISIBLE_ROWS.to_string(),
                    value: draft(),
                    oninput: move |evt| {
                        draft.set(evt.value());
                    },
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter && !evt.modifiers().shift() {
                            evt.prevent_default();
                            let text = draft().trim().to_string();
                            if text.is_empty() {
                                return;
                            }
                            on_submit.call(text);
                            draft.set(String::new());
                        }
                    },
                }

                div { class: "flex items-center justify-between mt-4",
                    div { class: "flex items-center gap-2",
                        button {
                            class: "p-2 text-neutral-300 hover:bg-white/10 hover:text-primary-50 rounded-full transition-colors",
                            r#type: "button",
                            "attach_file"
                        }
                        button {
                            class: "p-2 text-neutral-300 hover:bg-white/10 hover:text-primary-50 rounded-full transition-colors",
                            r#type: "button",
                            "image"
                        }
                        button {
                            class: "p-2 text-neutral-300 hover:bg-white/10 hover:text-primary-50 rounded-full transition-colors",
                            r#type: "button",
                            "mic"
                        }
                    }

                    button {
                        class: "bg-primary-200 text-secondary-950 h-12 w-12 flex items-center justify-center rounded-full shadow-lg shadow-black/40 hover:scale-105 active:scale-95 transition-all",
                        r#type: "submit",
                        "send"
                    }
                }
            }
        }
    }
}
