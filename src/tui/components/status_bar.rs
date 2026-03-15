use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn render(area: Rect, buf: &mut Buffer) {
    Paragraph::new("status")
        .block(Block::default().title("status").borders(Borders::ALL))
        .render(area, buf);
}
