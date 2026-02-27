use std::fmt::Write;

use chrono::{Local, Utc};
use serde::Deserialize;
use tracing::{Event, Subscriber};
use tracing_subscriber::{Layer, layer::Context};

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimestampMode {
    Local,
    #[default]
    Utc,
}

pub struct AppConsoleLayer {
    timestamp_mode: TimestampMode,
}

impl AppConsoleLayer {
    pub fn new(timestamp_mode: TimestampMode) -> Self {
        Self { timestamp_mode }
    }
}

impl<S> Layer<S> for AppConsoleLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();
        let mut visitor = EventFieldVisitor::default();

        event.record(&mut visitor);

        let timestamp = human_timestamp(self.timestamp_mode);
        let level = format_level(metadata.level());
        let location = format_location(metadata.file(), metadata.line());
        let message = visitor
            .message
            .as_deref()
            .unwrap_or_else(|| metadata.name());
        let fields = visitor.render_fields();

        if fields.is_empty() {
            println!(
                "{} {} {}{} - {}",
                timestamp,
                level,
                metadata.target(),
                location,
                message
            );
            return;
        }

        println!(
            "{} {} {}{} - {} | {}",
            timestamp,
            level,
            metadata.target(),
            location,
            message,
            fields
        );
    }
}

#[derive(Default)]
struct EventFieldVisitor {
    message: Option<String>,
    fields: Vec<(String, String)>,
}

impl tracing::field::Visit for EventFieldVisitor {
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.push_kv(field.name(), value.to_string());
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.push_kv(field.name(), value.to_string());
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.push_kv(field.name(), value.to_string());
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.push_kv(field.name(), value.to_owned());
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.push_kv(field.name(), format!("{value:?}"));
    }
}

impl EventFieldVisitor {
    fn push_kv(&mut self, name: &str, value: String) {
        if name == "message" {
            self.message = Some(value);
            return;
        }

        self.fields.push((name.to_owned(), value));
    }

    fn render_fields(&self) -> String {
        let mut rendered = String::new();

        for (index, (name, value)) in self.fields.iter().enumerate() {
            if index > 0 {
                rendered.push(' ');
            }

            let _ = write!(&mut rendered, "{}={}", name, value);
        }

        rendered
    }
}

fn human_timestamp(mode: TimestampMode) -> String {
    match mode {
        TimestampMode::Utc => Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        TimestampMode::Local => Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
    }
}

fn format_location(file: Option<&str>, line: Option<u32>) -> String {
    match (file, line) {
        (Some(file), Some(line)) => format!(" [{}:{}]", file, line),
        (Some(file), None) => format!(" [{}]", file),
        _ => String::new(),
    }
}

fn format_level(level: &tracing::Level) -> String {
    let level_text = format!("{:>5}", level.as_str());

    if std::env::var_os("NO_COLOR").is_some() {
        return level_text;
    }

    let color_code = match *level {
        tracing::Level::TRACE => "90",
        tracing::Level::DEBUG => "34",
        tracing::Level::INFO => "32",
        tracing::Level::WARN => "33",
        tracing::Level::ERROR => "31",
    };

    format!("\x1b[{}m{}\x1b[0m", color_code, level_text)
}
