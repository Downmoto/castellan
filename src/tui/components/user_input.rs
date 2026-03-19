//! dedicated user input component for chat.

use crate::tui::util::{
    dedicated_grey_colour, 
    dedicated_input_background_colour, 
    primary_colour, 
    wrapped_line_count,
};

use ratatui::{
    layout::Margin,
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

/// input widget state adapter for rendering typed content.
pub struct UserInputWidget<'a> {
    input: &'a str,
}

impl<'a> UserInputWidget<'a> {
    /// creates an input widget over the provided input text.
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    /// computes rendered height with borders and wrapped content.
    ///
    /// returned height includes one-cell padding on all sides.
    pub fn required_height(input: &str, width: u16) -> u16 {
        if width == 0 {
            return 1;
        }

        let content_width = width.saturating_sub(2).max(1) as usize;
        let input_with_cursor = if input.is_empty() {
            String::new()
        } else {
            format!("{input}█")
        };
        let wrapped_rows = wrapped_line_count(&input_with_cursor, content_width).max(1);

        wrapped_rows.saturating_add(2).min(u16::MAX as usize) as u16
    }
}

impl Widget for UserInputWidget<'_> {
    /// renders bordered wrapped input text with a live cursor glyph.
    ///
    /// when input is empty, a placeholder is shown after the cursor.
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::default()
            .bg(dedicated_input_background_colour())
            .render(area, buf);

        let cursor_style = Style::default().fg(primary_colour());
        let input_style = Style::default().fg(primary_colour());
        let placeholder_style = Style::default().fg(dedicated_grey_colour());

        let line = if self.input.is_empty() {
            Line::from(vec![
                Span::styled("█", cursor_style),
                Span::styled("type a message...", placeholder_style),
            ])
        } else {
            Line::from(vec![
                Span::styled(self.input.to_string(), input_style),
                Span::styled("█", cursor_style),
            ])
        };

        Paragraph::new(Text::from(line))
            .style(Style::default().bg(dedicated_input_background_colour()))
            .wrap(Wrap { trim: false })
            .render(
                area.inner(Margin {
                    horizontal: 1,
                    vertical: 1,
                }),
                buf,
            );
    }
}
