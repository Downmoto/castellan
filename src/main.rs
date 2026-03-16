use castellan::logging::prelude::*;
use castellan::llm;
use castellan::input::{KeyAction, KeyCommand, KeybindResolver};
use castellan::settings::prelude::*;
use castellan::tui::{app::Castellan, prelude::*};

use crossterm::event::{self as c_event, Event, KeyEventKind};
use dotenv::dotenv;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{event, span, Level};

enum AppEvent {
    AssistantReady { tab_index: usize, message: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let settings = settings();
    let _subscriber = logging_init(settings.app_log().level, settings.app_log().timestamp_mode);

    let _guard = span!(Level::INFO, "castellan_global").entered();
    event!(Level::INFO, "App start");

    if used_default_settings() {
        event!(Level::WARN, "Failed to parse configuration; using defaults");
    }

    let mut terminal = init_terminal()?;
    let mut app = Castellan::default();
    let key_resolver = KeybindResolver::new(settings.keybinds());
    let scroll_line_step = settings.scroll().line_step;
    let scroll_page_step = settings.scroll().page_step;
    let (tx, mut rx) = mpsc::channel::<AppEvent>(64);
    event!(Level::INFO, "App initialized");

    loop {
        let area = terminal.size()?;
        app.update_viewport_from_area(area.into());

        terminal.draw(|frame| {
            frame.render_widget(&app, frame.area());
        })?;

        while let Ok(app_event) = rx.try_recv() {
            match app_event {
                AppEvent::AssistantReady { tab_index, message } => {
                    app.push_assistant_message_for_tab(tab_index, message)
                }
            }
        }

        if c_event::poll(Duration::from_millis(100))? && let Event::Key(key) = c_event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key_resolver.resolve(key, app.input_mode()) {
                KeyAction::Command(command) => match command {
                    KeyCommand::ExitApp => break,
                    KeyCommand::EnterInputMode => app.enter_input_mode(),
                    KeyCommand::ExitInputMode => app.exit_input_mode(),
                    KeyCommand::NewChatTab => app.new_chat_tab(),
                    KeyCommand::NextTab => app.next_tab(),
                    KeyCommand::PrevTab => app.prev_tab(),
                    KeyCommand::Backspace => app.backspace(),
                    KeyCommand::CloseCurrentTab => app.close_current_tab(),
                    KeyCommand::ScrollUp => app.scroll_up(scroll_line_step),
                    KeyCommand::ScrollDown => app.scroll_down(scroll_line_step),
                    KeyCommand::PageUp => app.scroll_up(scroll_page_step),
                    KeyCommand::PageDown => app.scroll_down(scroll_page_step),
                    KeyCommand::ScrollToBottom => app.scroll_to_bottom(),
                    KeyCommand::Submit => {
                        if let Some((tab_index, message)) = app.take_input_for_submit() {
                            app.push_user_message(message.clone());
                            let sender = tx.clone();

                            tokio::spawn(async move {
                                let reply = match llm::generate_reply(&message).await {
                                    Ok(content) => content,
                                    Err(error) => format!(
                                        "llm request failed: {error}. set OPENAI_API_KEY and optional CAST_LLM_MODEL."
                                    ),
                                };

                                let _ = sender
                                    .send(AppEvent::AssistantReady {
                                        tab_index,
                                        message: reply,
                                    })
                                    .await;
                            });
                        }
                    }
                },
                KeyAction::InsertChar(ch) => app.push_char(ch),
                KeyAction::Noop => {}
            }
        }
    }

    deinit_terminal(&mut terminal)?;
    event!(Level::INFO, "Exiting castellan");

    Ok(())
}