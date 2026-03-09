use castellan::logging::prelude::*;
use castellan::settings::prelude::*;

use tracing::{Level, event, span};
use dotenv::dotenv;



use std::time::Duration;
use crossterm::{
    event::{self as cevent, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use tokio::sync::mpsc;

enum AppEvent {
    FetchComplete(String),
    Error(String),
}

struct App {
    result: String,
    loading: bool,
}

impl App {
    fn new() -> Self {
        Self {
            result: "Press 'f' to fetch data".to_string(),
            loading: false,
        }
    }
}

async fn fetch_data(tx: mpsc::Sender<AppEvent>) {
    // Simulate network latency / heavy async work
    tokio::time::sleep(Duration::from_secs(2)).await;

    match some_async_work().await {
        Ok(data) => tx.send(AppEvent::FetchComplete(data)).await.ok(),
        Err(e)   => tx.send(AppEvent::Error(e)).await.ok(),
    };
}

async fn some_async_work() -> Result<String, String> {
    // Replace with reqwest, sqlx, etc.
    Ok("✓ Data fetched successfully after 2 seconds!".to_string())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // slated for removal as dep

    let settings = settings();
    let _subscriber = logging_init(settings.app_log().level, settings.app_log().timestamp_mode);

    let _guard = span!(Level::INFO, "castellan_global").entered();
    event!(Level::INFO, "App start");

    if used_default_settings() {
        event!(Level::WARN, "Failed to parse configuration; using defaults")
    }


    // here
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Channel: async tasks → UI
    let (tx, mut rx) = mpsc::channel::<AppEvent>(32);

    let mut app = App::new();

    loop {
        // 1. Draw
        terminal.draw(|frame| {
            let area = frame.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            // Status box
            let status = if app.loading { "⏳ Loading..." } else { "Idle" };
            let status_widget = Paragraph::new(status)
                .block(Block::default().borders(Borders::ALL).title("Status"))
                .alignment(Alignment::Center)
                .style(Style::default().fg(if app.loading { Color::Yellow } else { Color::Green }));

            // Result box
            let result_widget = Paragraph::new(app.result.as_str())
                .block(Block::default().borders(Borders::ALL).title("Result"))
                .alignment(Alignment::Center);

            frame.render_widget(status_widget, chunks[0]);
            frame.render_widget(result_widget, chunks[1]);
        })?;

        // 2. Check for messages from async tasks (non-blocking)
        while let Ok(event) = rx.try_recv() {
            match event {
                AppEvent::FetchComplete(data) => {
                    app.result = data;
                    app.loading = false;
                }
                AppEvent::Error(e) => {
                    app.result = format!("Error: {e}");
                    app.loading = false;
                }
            }
        }

        // 3. Poll keyboard events (with timeout so we don't block the channel check)
        if cevent::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = cevent::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('f') if !app.loading => {
                        app.loading = true;
                        app.result = "Fetching...".to_string();
                        // Spawn the async task — it will send back via `tx`
                        tokio::spawn(fetch_data(tx.clone()));
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())

}