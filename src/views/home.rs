use dioxus::prelude::*;

use crate::components::{Chat, UserInput};

#[component]
pub fn HomeView() -> Element {
    let mut messages = use_signal(|| vec![]);

    rsx! {
        div { class: "flex h-full w-full flex-col rounded-xl border border-secondary-200 bg-primary-50 p-4",
            div { class: "mb-3 border-b border-secondary-200 pb-2",
                h1 { class: "text-base font-semibold text-secondary-900", "castellan chat" }
            }

            div { class: "min-h-0 flex-1",
                Chat { messages }
            }

            UserInput {
                on_submit: move |text| {
                    messages.with_mut(|items| items.push(text));
                },
            }
        }
    }
}