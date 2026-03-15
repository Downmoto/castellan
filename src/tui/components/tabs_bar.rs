use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn render(area: Rect, buf: &mut Buffer) {
    Paragraph::new("tabs")
        .block(Block::default().title("tabs").borders(Borders::ALL))
        .render(area, buf);
}
