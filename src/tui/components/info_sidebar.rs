use ratatui::{
    prelude::{Buffer, Rect}, style::{Style, Stylize}, widgets::{Block, Borders, Paragraph, Widget}
};

use crate::tui::util::secondary_colour;

pub struct InfoSidebar;

impl InfoSidebar {
    pub fn new() -> Self {
        Self
    }
}

impl Widget for InfoSidebar {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized 
    {
        let frame_block = Block::default()
            .border_style(Style::default().fg(secondary_colour()))
            .borders(Borders::ALL)
            .bg(secondary_colour());
        
        let content_area = frame_block.inner(area);
        frame_block.render(area, buf);

        Paragraph::new("metadata / useful info").render(content_area, buf);
    }
}