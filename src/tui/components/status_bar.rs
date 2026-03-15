use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

pub fn render(area: Rect, buf: &mut Buffer, status_text: &str) {
    Paragraph::new(status_text)
        .block(Block::default().title("status").borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .render(area, buf);
}
