use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

use super::components::{
    chat::{ChatMessage, ChatWidget},
    info_sidebar, status_bar, tabs_bar,
};

#[derive(Default)]
pub struct Castellan {
    input: String,
    messages: Vec<ChatMessage>,
    scroll_from_bottom: usize,
    transcript_viewport_lines: usize,
}

impl Castellan {
    fn max_scroll_from_bottom(&self) -> usize {
        let total_lines = if self.messages.is_empty() {
            1
        } else {
            self.messages.len()
        };
        total_lines.saturating_sub(self.transcript_viewport_lines.max(1))
    }

    fn clamp_scroll(&mut self) {
        self.scroll_from_bottom = self.scroll_from_bottom.min(self.max_scroll_from_bottom());
    }

    fn transcript_height_from_area(area: Rect) -> usize {
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

        let chat_area = columns[0];
        let inner_height = chat_area.height.saturating_sub(2);
        inner_height.saturating_sub(1).max(1) as usize
    }

    pub fn update_viewport_from_area(&mut self, area: Rect) {
        self.transcript_viewport_lines = Self::transcript_height_from_area(area);
        self.clamp_scroll();
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }

    pub fn scroll_from_bottom(&self) -> usize {
        self.scroll_from_bottom
    }

    pub fn push_char(&mut self, ch: char) {
        self.input.push(ch);
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn take_input_for_submit(&mut self) -> Option<String> {
        let candidate = self.input.trim().to_string();
        if candidate.is_empty() {
            return None;
        }

        self.input.clear();
        Some(candidate)
    }

    pub fn push_user_message(&mut self, content: String) {
        self.messages.push(ChatMessage {
            sender: "you".to_string(),
            content,
        });
        self.scroll_to_bottom();
    }

    pub fn push_assistant_message(&mut self, content: String) {
        self.messages.push(ChatMessage {
            sender: "assistant".to_string(),
            content,
        });
        self.scroll_to_bottom();
    }

    pub fn scroll_up(&mut self, lines: usize) {
        let next = self.scroll_from_bottom.saturating_add(lines);
        self.scroll_from_bottom = next.min(self.max_scroll_from_bottom());
    }

    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_from_bottom = self.scroll_from_bottom.saturating_sub(lines);
        self.clamp_scroll();
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_from_bottom = 0;
    }

    pub fn status_text(&self) -> String {
        let scroll = if self.scroll_from_bottom == 0 {
            "scroll: bottom".to_string()
        } else {
            format!("scroll: {} lines up", self.scroll_from_bottom)
        };

        format!(
            "{} | keys: up/down pgup/pgdn end | submit: enter | quit: esc/ctrl+c",
            scroll
        )
    }
}

impl Widget for &Castellan {
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

        ChatWidget::new(self.input(), self.messages(), self.scroll_from_bottom())
            .render(columns[0], buf);

        info_sidebar::render(columns[1], buf);
        tabs_bar::render(page[1], buf);
        let status_text = self.status_text();
        status_bar::render(page[2], buf, &status_text);
    }
}
