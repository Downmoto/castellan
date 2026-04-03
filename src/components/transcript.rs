use dioxus::prelude::*;

#[component]
pub fn TranscriptComponent(messages: Signal<Vec<String>>) -> Element {
    let current_messages = messages();

    rsx! {
        div { class: "h-full w-full overflow-y-auto rounded-lg rounded-b-md bg-neutral-900 flex flex-col",
            if current_messages.is_empty() {
                div { class: "h-full flex flex-col",
                    div { class: "grow flex flex-col items-center justify-center px-6 pt-32 pb-16",
                        div { class: "w-full max-w-3xl flex flex-col items-center space-y-10 mt-auto mb-auto",
                            div { class: "text-center space-y-4",
                                h2 { class: "font-headline text-5xl font-extrabold text-primary-50 tracking-tight",
                                    "Good morning."
                                }
                                p { class: "font-body text-xl text-neutral-300 font-light max-w-lg mx-auto",
                                    "Your workspace is quiet and ready. Let's curate something extraordinary."
                                }
                            }
                        }
                    }
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
