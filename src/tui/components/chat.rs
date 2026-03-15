use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn render(area: Rect, buf: &mut Buffer) {
    Paragraph::new("chat")
        .block(Block::default().title("chat").borders(Borders::ALL))
        .render(area, buf);
}
