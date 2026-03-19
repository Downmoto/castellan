//! assistant response message rendering helpers.

use crate::tui::util::dedicated_dark_grey_colour;

use ratatui::text::{Line, Span};

const RESPONSE_LINE_PREFIX: &str = "│";

/// assistant response adapter for chat transcript rendering.
///
/// each visual row starts with a fixed prefix marker so assistant messages are
/// visually distinct from user messages.
pub struct ResponseMessageWidget<'a> {
    content: &'a str,
    width: usize,
}

impl<'a> ResponseMessageWidget<'a> {
    /// creates a response message view over assistant-authored content.
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
        let lines: Vec<String> = self
            .wrapped_content_lines()
            .into_iter()
            .map(|line| format!("{RESPONSE_LINE_PREFIX}{line}"))
            .collect();

        if lines.is_empty() {
            vec![RESPONSE_LINE_PREFIX.to_string()]
        } else {
            lines
        }
    }

    /// returns styled rows for transcript rendering.
    ///
    /// this must stay row-count compatible with `plain_rows`.
    pub fn styled_rows(self) -> Vec<Line<'static>> {
        let border_style = ratatui::style::Style::default().fg(dedicated_dark_grey_colour());
        let lines: Vec<Line<'static>> = self
            .wrapped_content_lines()
            .into_iter()
            .map(|line| {
                Line::from(vec![
                    Span::styled(RESPONSE_LINE_PREFIX.to_string(), border_style),
                    Span::raw(line),
                ])
            })
            .collect();

        if lines.is_empty() {
            vec![Line::from(vec![Span::styled(
                RESPONSE_LINE_PREFIX.to_string(),
                border_style,
            )])]
        } else {
            lines
        }
    }

    fn wrapped_content_lines(&self) -> Vec<String> {
        let content_width = self.width.saturating_sub(RESPONSE_LINE_PREFIX.chars().count());
        let content_width = content_width.max(1);
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

        rows
    }
}
