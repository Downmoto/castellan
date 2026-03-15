//! app shell composition and event-facing api.
//! this module owns top-level layout and delegates chat behavior.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

use super::components::{
    chat::{ChatState, ChatWidget},
    info_sidebar, status_bar, tabs_bar,
};

#[derive(Default)]
/// root tui state for cross-component composition.
pub struct Castellan {
    chat: ChatState,
}

impl Castellan {
    /// updates chat viewport metrics from the current terminal area.
    pub fn update_viewport_from_area(&mut self, area: Rect) {
        let page = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(area);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Length(30)])
            .split(page[0]);

        self.chat.set_viewport_from_area(columns[0]);
    }
}

impl Castellan {
    /// appends a typed character to the chat input buffer.
    pub fn push_char(&mut self, ch: char) {
        self.chat.push_char(ch);
    }

    /// removes the last character from the chat input buffer.
    pub fn backspace(&mut self) {
        self.chat.backspace();
    }

    /// returns a trimmed message if submit is valid and clears input.
    pub fn take_input_for_submit(&mut self) -> Option<String> {
        self.chat.take_input_for_submit()
    }

    /// appends a user-authored message to the transcript.
    pub fn push_user_message(&mut self, content: String) {
        self.chat.push_user_message(content);
    }

    /// appends an assistant-authored message to the transcript.
    pub fn push_assistant_message(&mut self, content: String) {
        self.chat.push_assistant_message(content);
    }

    /// scrolls transcript upward by the requested number of wrapped lines.
    pub fn scroll_up(&mut self, lines: usize) {
        self.chat.scroll_up(lines);
    }

    /// scrolls transcript downward by the requested number of wrapped lines.
    pub fn scroll_down(&mut self, lines: usize) {
        self.chat.scroll_down(lines);
    }

    /// resets transcript scroll to the newest message.
    pub fn scroll_to_bottom(&mut self) {
        self.chat.scroll_to_bottom();
    }

    /// builds status text for the shared status bar component.
    pub fn status_text(&self) -> String {
        self.chat.status_text()
    }
}

impl Widget for &Castellan {
    /// renders the full app page and delegates chat drawing.
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Block::default()
            .style(Style::default().bg(Color::Black))
            .render(area, buf);

        let page = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(area);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Length(30)])
            .split(page[0]);

        ChatWidget::new(&self.chat).render(columns[0], buf);

        info_sidebar::render(columns[1], buf);
        tabs_bar::render(page[1], buf);
        let status_text = self.status_text();
        status_bar::render(page[2], buf, &status_text);
    }
}
