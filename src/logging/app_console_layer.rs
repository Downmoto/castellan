use std::fmt::Write;

use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

pub struct AppConsoleLayer;

impl AppConsoleLayer {
    pub fn new() -> Self {
        Self {}
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

        if visitor.fields.is_empty() {
            println!("[{} {}] {}", metadata.level(), metadata.target(), metadata.name());
            return;
        }

        println!(
            "[{} {}] -{}- {}",
            metadata.level(),
            metadata.target(),
            metadata.name(),
            visitor.fields
        );
    }
}

#[derive(Default)]
struct EventFieldVisitor {
    fields: String,
}

impl tracing::field::Visit for EventFieldVisitor {
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.push_kv(field.name(), value);
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.push_kv(field.name(), value);
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.push_kv(field.name(), value);
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.push_kv(field.name(), value);
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.push_kv(field.name(), format_args!("{:?}", value));
    }
}

impl EventFieldVisitor {
    fn push_kv(&mut self, name: &str, value: impl std::fmt::Display) {
        if !self.fields.is_empty() {
            self.fields.push(' ');
        }

        let _ = write!(&mut self.fields, "{}={}", name, value);
    }
}