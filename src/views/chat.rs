use dioxus::prelude::*;

use crate::components::{ChatMessage, TranscriptComponent, UserInputComponent};
use crate::llms::generate_reply;

#[component]
pub fn ChatView() -> Element {
    let mut messages = use_signal(|| vec![]);
    let is_empty = messages().is_empty();

    rsx! {
        if is_empty {
            div { class: "flex h-full w-full flex-col items-center justify-center rounded-xl px-6",
                div { class: "w-full max-w-3xl text-center space-y-4",
                    h2 { class: "font-headline text-5xl font-extrabold text-primary-50 tracking-tight",
                        "Good morning."
                    }
                    p { class: "font-body text-xl text-neutral-300 font-light max-w-lg mx-auto",
                        "Your workspace is quiet and ready. Let's curate something extraordinary."
                    }
                }

                div { class: "mt-10 w-full max-w-3xl",
                    UserInputComponent {
                        on_submit: move |text: String| {
                            messages.with_mut(|items| items.push(ChatMessage::User(text.clone())));

                            let mut messages = messages;
                            spawn(async move {
                                let reply = match generate_reply(&text).await {
                                    Ok(content) => content,
                                    Err(error) => format!("error: {error}"),
                                };

                                messages.with_mut(|items| items.push(ChatMessage::Assistant(reply)));
                            });
                        },
                    }
                }
            }
        } else {
            div { class: "flex h-full w-full flex-col rounded-xl",
                div { class: "min-h-0 flex-1",
                    TranscriptComponent { messages }
                }

                div { class: "mt-3 w-full max-w-3xl mx-auto",
                    UserInputComponent {
                        on_submit: move |text: String| {
                            messages.with_mut(|items| items.push(ChatMessage::User(text.clone())));

                            let mut messages = messages;
                            spawn(async move {
                                let reply = match generate_reply(&text).await {
                                    Ok(content) => content,
                                    Err(error) => format!("error: {error}"),
                                };

                                messages.with_mut(|items| items.push(ChatMessage::Assistant(reply)));
                            });
                        },
                    }
                }
            }
        }
    }
}