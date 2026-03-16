use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn render(area: Rect, buf: &mut Buffer) {
    Paragraph::new("metadata / useful info")
        .block(Block::default().title("info").borders(Borders::ALL))
        .render(area, buf);
}
