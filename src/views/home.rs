use dioxus::prelude::*;

use crate::components::{ChatComponent, UserInputComponent};

#[component]
pub fn HomeView() -> Element {
    let mut messages = use_signal(|| vec![]);

    rsx! {
        div { class: "flex h-full w-full flex-col rounded-xl border border-secondary-200 bg-primary-50 p-4",
            div { class: "min-h-0 flex-1",
                ChatComponent { messages }
            }

            UserInputComponent {
                on_submit: move |text| {
                    messages.with_mut(|items| items.push(text));
                },
            }
        }
    }
}