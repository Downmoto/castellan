use castellan::logging::prelude::*;
use castellan::llm;
use castellan::settings::prelude::*;
use castellan::tui::{app::Castellan, prelude::*};

use crossterm::event::{self as cevent, Event, KeyCode, KeyEventKind, KeyModifiers};
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
    let (tx, mut rx) = mpsc::channel::<AppEvent>(64);

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

        if cevent::poll(Duration::from_millis(100))? && let Event::Key(key) = cevent::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Esc => break,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.new_chat_tab()
                }
                KeyCode::Tab => app.next_tab(),
                KeyCode::BackTab => app.prev_tab(),
                KeyCode::Char(ch) => app.push_char(ch),
                KeyCode::Backspace => app.backspace(),
                KeyCode::Up => app.scroll_up(1),
                KeyCode::Down => app.scroll_down(1),
                KeyCode::PageUp => app.scroll_up(10),
                KeyCode::PageDown => app.scroll_down(10),
                KeyCode::End => app.scroll_to_bottom(),
                KeyCode::Enter => {
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
                _ => {}
            }
        }
    }

    deinit_terminal(&mut terminal)?;
    event!(Level::INFO, "Exiting castellan");

    Ok(())
}