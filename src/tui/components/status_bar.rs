use ratatui::{
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::tui::util::secondary_colour;

pub fn render(area: Rect, buf: &mut Buffer, status_text: &str) {
    let frame_block = Block::default()
        .border_style(Style::default().fg(secondary_colour()))
        .borders(Borders::ALL)
        .bg(secondary_colour());

    let content_area = frame_block.inner(area);
    frame_block.render(area, buf);

    Paragraph::new(status_text)
        .wrap(Wrap { trim: true })
        .render(content_area, buf);
}
