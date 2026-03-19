use castellan::logging::prelude::*;
use castellan::settings::prelude::*;
use castellan::input::{KeyAction, KeybindResolver};
use castellan::llm::{AssistantReply, LlmService};
use castellan::tui::{app::{Castellan, CommandResult}, prelude::*};

use crossterm::event::{self as c_event, Event, KeyEventKind};
use dotenv::dotenv;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{event, span, Level};

/// drains all ready assistant replies and appends them to their destination tabs.
///
/// this keeps ui rendering responsive by using non-blocking receive semantics.
fn drain_assistant_replies(app: &mut Castellan, rx: &mut mpsc::Receiver<AssistantReply>) {
    while let Ok(reply) = rx.try_recv() {
        app.push_assistant_message_for_tab(reply.tab_index, reply.message)
    }
}

#[tokio::main]
/// runs the interactive terminal loop and coordinates ui, input, and llm tasks.
///
/// lifecycle flow:
/// - load environment and typed settings.
/// - initialize logging and terminal resources.
/// - process key events into app commands.
/// - dispatch submit events to asynchronous llm workers.
/// - drain completed assistant replies into tab transcripts.
/// - restore terminal state before exit.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let settings = settings();
    let _subscriber = logging_init(settings.app_log.level, settings.app_log.timestamp_mode);

    let _guard = span!(Level::INFO, "castellan_global").entered();
    event!(Level::INFO, "App start");

    if used_default_settings() {
        event!(Level::WARN, "Failed to parse configuration; using defaults");
    }

    let mut terminal = init_terminal()?;
    let mut app = Castellan::default();
    let key_resolver = KeybindResolver::new(&settings.keybinds);
    let scroll_line_step = settings.scroll.line_step;
    let scroll_page_step = settings.scroll.page_step;
    let llm_service = LlmService::new();
    let (tx, mut rx) = mpsc::channel::<AssistantReply>(64);
    event!(Level::INFO, "App initialized");

    loop {
        let area = terminal.size()?;
        app.update_viewport_from_area(area.into());

        terminal.draw(|frame| {
            frame.render_widget(&app, frame.area());
        })?;

        drain_assistant_replies(&mut app, &mut rx);

        if c_event::poll(Duration::from_millis(100))? && let Event::Key(key) = c_event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if app.handle_rename_key_event(key) {
                continue;
            }

            match key_resolver.resolve(key, app.input_mode) {
                KeyAction::Command(command) => {
                    match app.apply_command(command, scroll_line_step, scroll_page_step) {
                        CommandResult::Exit => break,
                        CommandResult::None => {}
                        CommandResult::Submit { tab_index, message } => {
                            llm_service.request_reply(tx.clone(), tab_index, message);
                        }
                    }
                }
                KeyAction::InsertChar(ch) => app.push_char(ch),
                KeyAction::Noop => {}
            }
        }
    }

    deinit_terminal(&mut terminal)?;
    event!(Level::INFO, "Exiting castellan");

    Ok(())
}