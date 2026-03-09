use castellan::tui::prelude::deinit_terminal;
use castellan::{logging::prelude::*, tui::prelude::init_terminal};
use castellan::settings::prelude::*;

use tracing::{Level, event, span};
use dotenv::dotenv;

use std::time::Duration;
use crossterm::{
    event::{self as cevent, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use tokio::sync::mpsc;

#[derive(Clone, Copy)]
enum TaskKind {
    Primary,
    Secondary,
}

enum AppEvent {
    FetchComplete { task: TaskKind, data: String },
    Error { task: TaskKind, message: String },
}

struct TaskPanel {
    title: &'static str,
    trigger: char,
    result: String,
    loading: bool,
}

struct App {
    primary: TaskPanel,
    secondary: TaskPanel,
}

impl App {
    fn new() -> Self {
        Self {
            primary: TaskPanel {
                title: "task f",
                trigger: 'f',
                result: "Press 'f' to fetch data".to_string(),
                loading: false,
            },
            secondary: TaskPanel {
                title: "task g",
                trigger: 'g',
                result: "Press 'g' to fetch data".to_string(),
                loading: false,
            },
        }
    }

    fn panel_mut(&mut self, task: TaskKind) -> &mut TaskPanel {
        match task {
            TaskKind::Primary => &mut self.primary,
            TaskKind::Secondary => &mut self.secondary,
        }
    }
}

async fn fetch_data(tx: mpsc::Sender<AppEvent>, task: TaskKind, delay: Duration, source: &'static str) {
    // Simulate network latency / heavy async work
    tokio::time::sleep(delay).await;

    match some_async_work(source).await {
        Ok(data) => tx.send(AppEvent::FetchComplete { task, data }).await.ok(),
        Err(message) => tx.send(AppEvent::Error { task, message }).await.ok(),
    };
}

async fn some_async_work(source: &str) -> Result<String, String> {
    // Replace with reqwest, sqlx, etc.
    Ok(format!("{source}: fetched data successfully"))
}

fn render_task_panel(frame: &mut Frame<'_>, area: ratatui::layout::Rect, panel: &TaskPanel) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    let status = if panel.loading { "loading..." } else { "idle" };
    let status_widget = Paragraph::new(status)
        .block(Block::default().borders(Borders::ALL).title(format!("{} status", panel.title)))
        .alignment(Alignment::Center)
        .style(Style::default().fg(if panel.loading { Color::Yellow } else { Color::Green }));

    let result_widget = Paragraph::new(panel.result.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{} result (press '{}')", panel.title, panel.trigger)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(status_widget, chunks[0]);
    frame.render_widget(result_widget, chunks[1]);
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
    let mut terminal = init_terminal().unwrap();


    // Channel: async tasks → UI
    let (tx, mut rx) = mpsc::channel::<AppEvent>(32);

    let mut app = App::new();
    // let a = Castellan::default();

    loop {
        // 1. Draw
        terminal.draw(|frame| {
            let area = frame.area();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            render_task_panel(frame, chunks[0], &app.primary);
            render_task_panel(frame, chunks[1], &app.secondary);
        })?;

        // 2. Check for messages from async tasks (non-blocking)
        while let Ok(event) = rx.try_recv() {
            match event {
                AppEvent::FetchComplete { task, data } => {
                    let panel = app.panel_mut(task);
                    panel.result = data;
                    panel.loading = false;
                }
                AppEvent::Error { task, message } => {
                    let panel = app.panel_mut(task);
                    panel.result = format!("error: {message}");
                    panel.loading = false;
                }
            }
        }

        // 3. Poll keyboard events (with timeout so we don't block the channel check)
        if cevent::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = cevent::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('f') if !app.primary.loading => {
                        app.primary.loading = true;
                        app.primary.result = "fetching...".to_string();
                        // spawn independent task f and send completion over the channel
                        tokio::spawn(fetch_data(
                            tx.clone(),
                            TaskKind::Primary,
                            Duration::from_secs(2),
                            "task f",
                        ));
                    }
                    KeyCode::Char('g') if !app.secondary.loading => {
                        app.secondary.loading = true;
                        app.secondary.result = "fetching...".to_string();
                        // spawn independent task g so both async tasks can overlap
                        tokio::spawn(fetch_data(
                            tx.clone(),
                            TaskKind::Secondary,
                            Duration::from_secs(3),
                            "task g",
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    let _ = deinit_terminal(&mut terminal);

    Ok(())

}