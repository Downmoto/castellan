//! user request message rendering helpers.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::tui::util::secondary_colour;

const HORIZONTAL_PADDING: usize = 1;

/// request message adapter for chat transcript rendering.
pub struct RequestMessageWidget<'a> {
    content: &'a str,
    width: usize,
}

impl<'a> RequestMessageWidget<'a> {
    /// creates a request message view over user-authored content.
    pub fn new(content: &'a str, width: usize) -> Self {
        Self {
            content,
            width: width.max(1),
        }
    }

    /// returns plain-text rows used for wrapped line counting.
    pub fn plain_rows(self) -> Vec<String> {
        let mut rows = Vec::new();
        let width = self.row_width();
        let inner_width = width.saturating_sub(2);
        let spacer = " ".repeat(width);

        rows.push(spacer.clone());

        for line in self.content_lines() {
            rows.push(format!(" {line:<inner_width$} ", inner_width = inner_width));
        }

        rows.push(spacer);
        rows
    }

    /// returns styled rows for transcript rendering.
    pub fn styled_rows(self) -> Vec<Line<'static>> {
        let mut rows = Vec::new();
        let width = self.row_width();
        let inner_width = width.saturating_sub(2);
        let style = Style::default().fg(Color::White).bg(secondary_colour());
        let spacer = " ".repeat(width);

        rows.push(Line::from(vec![Span::styled(spacer.clone(), style)]));

        for line in self.content_lines() {
            rows.push(Line::from(vec![Span::styled(
                format!(" {line:<inner_width$} ", inner_width = inner_width),
                style,
            )]));
        }

        rows.push(Line::from(vec![Span::styled(spacer, style)]));
        rows
    }

    fn content_lines(&self) -> Vec<&str> {
        let lines: Vec<&str> = self.content.split('\n').collect();
        if lines.is_empty() {
            vec![""]
        } else {
            lines
        }
    }

    fn row_width(&self) -> usize {
        let content_width = self
            .content_lines()
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        let min_content_width = content_width.max(HORIZONTAL_PADDING);
        let min_row_width = min_content_width.saturating_add(2);

        self.width.max(min_row_width)
    }
}
