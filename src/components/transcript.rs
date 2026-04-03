use dioxus::prelude::*;
use comrak::{markdown_to_html, Options};

#[derive(Clone, Debug, PartialEq)]
pub enum ChatMessage {
    User(String),
    Assistant(String),
}

impl ChatMessage {
    fn is_assistant(&self) -> bool {
        matches!(self, Self::Assistant(_))
    }

    fn content(&self) -> &str {
        match self {
            Self::User(content) | Self::Assistant(content) => content,
        }
    }
}

fn render_markdown(content: &str) -> String {
    let mut options = Options::default();
    // keep raw html escaped for safer rendering
    options.render.unsafe_ = false;
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.render.hardbreaks = true;

    // normalize newlines for consistent fenced code parsing
    let normalized = content.replace("\r\n", "\n");
    markdown_to_html(&normalized, &options)
}

#[component]
pub fn TranscriptComponent(messages: Signal<Vec<ChatMessage>>) -> Element {
    let current_messages = messages();

    rsx! {
        div { class: "h-full w-full overflow-y-auto rounded-lg rounded-b-md bg-neutral-900 flex flex-col-reverse",
            if current_messages.is_empty() {
                div { class: "h-full flex flex-col",
                    div { class: "grow flex flex-col items-center justify-center px-6 pt-32 pb-16",
                        div { class: "w-full max-w-3xl flex flex-col items-center space-y-10 mt-auto mb-auto",
                            // Empty State placeholder text
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

            for (idx , message) in current_messages.iter().enumerate().rev() {
                div { key: "{idx}", class: "mb-2.5 flex",
                    if message.is_assistant() {
                        div {
                            class: "markdown-content max-w-[85%] px-1 py-2 text-sm leading-relaxed text-primary-50 wrap-break-word",
                            dangerous_inner_html: render_markdown(message.content()),
                        }
                    } else {
                        p { class: "max-w-[85%] rounded-md bg-secondary-700 px-3 py-2 text-sm leading-relaxed text-primary-50 whitespace-pre-wrap wrap-break-word",
                            {message.content()}
                        }
                    }
                }
            }
        }
    }
}
