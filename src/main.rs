use castellan::logging::prelude::*;
use castellan::settings::prelude::*;
use castellan::tui::{app::Castellan, prelude::*};

use crossterm::event::{self as cevent, Event, KeyCode};
use dotenv::dotenv;
use std::time::Duration;
use tracing::{event, span, Level};

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
    let app = Castellan;

    loop {
        terminal.draw(|frame| {
            frame.render_widget(&app, frame.area());
        })?;

        if cevent::poll(Duration::from_millis(100))? && let Event::Key(key) = cevent::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    deinit_terminal(&mut terminal)?;
    event!(Level::INFO, "Exiting castellan");

    Ok(())
}