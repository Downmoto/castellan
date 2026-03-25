use dioxus::prelude::*;

use crate::components::{Chat, UserInput};

#[component]
pub fn ChatView() -> Element {
    let mut messages = use_signal(|| vec![]);

    rsx! {
        div { class: "w-full max-w-xl rounded-lg border border-neutral-700 bg-neutral-950 p-4",
            h1 { class: "mb-3 text-lg font-semibold text-neutral-100", "chat" }

            Chat { messages }

            UserInput {
                on_submit: move |text| {
                    messages.with_mut(|items| items.push(text));
                },
            }
        }
    }
}
