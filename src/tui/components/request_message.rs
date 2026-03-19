//! user request message rendering helpers.

use crate::tui::util::secondary_colour;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

const INNER_HORIZONTAL_PADDING: usize = 1;

/// request message adapter for chat transcript rendering.
///
/// this widget exposes both plain/styled row builders and a direct widget
/// renderer so transcript logic can either compose rows or render directly.
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
    ///
    /// this must stay row-count compatible with `styled_rows`.
    pub fn plain_rows(self) -> Vec<String> {
        let width = self.width;
        let inner_width = width.saturating_sub(2);
        let mut rows = Vec::new();
        let spacer = " ".repeat(width);

        rows.push(spacer.clone());

        for line in self.wrapped_content_lines() {
            rows.push(format!(" {line:<inner_width$} ", inner_width = inner_width));
        }

        rows.push(spacer);
        rows
    }

    /// returns styled rows for transcript rendering.
    ///
    /// this must stay row-count compatible with `plain_rows`.
    pub fn styled_rows(self) -> Vec<Line<'static>> {
        let width = self.width;
        let inner_width = width.saturating_sub(2);
        let mut rows = Vec::new();
        let style = Style::default().fg(Color::White).bg(secondary_colour());
        let spacer = " ".repeat(width);

        rows.push(Line::from(vec![Span::styled(spacer.clone(), style)]));

        for line in self.wrapped_content_lines() {
            rows.push(Line::from(vec![Span::styled(
                format!(" {line:<inner_width$} ", inner_width = inner_width),
                style,
            )]));
        }

        rows.push(Line::from(vec![Span::styled(spacer, style)]));
        rows
    }

    fn wrapped_content_lines(&self) -> Vec<String> {
        let content_width = self
            .width
            .saturating_sub(INNER_HORIZONTAL_PADDING.saturating_mul(2))
            .max(1);
        let mut rows = Vec::new();

        for source_line in self.content.split('\n') {
            if source_line.is_empty() {
                rows.push(String::new());
                continue;
            }

            let mut chunk = String::new();
            let mut chunk_width = 0usize;

            for ch in source_line.chars() {
                chunk.push(ch);
                chunk_width += 1;

                if chunk_width >= content_width {
                    rows.push(std::mem::take(&mut chunk));
                    chunk_width = 0;
                }
            }

            if !chunk.is_empty() {
                rows.push(chunk);
            }
        }

        if rows.is_empty() {
            vec![String::new()]
        } else {
            rows
        }
    }
}