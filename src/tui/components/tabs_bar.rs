use ratatui::{
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::tui::util::{dedicated_black_colour, dedicated_grey_colour, primary_colour, secondary_colour};

pub struct TabsBar<'a> {
    tabs: &'a [String],
    active_tab: usize,
}

impl<'a> TabsBar<'a> {
    pub fn new(tabs: &'a [String], active_tab: usize) -> Self {
        Self { tabs, active_tab }
    }

    fn tab_lines(&self) -> Vec<Line<'static>> {
        if self.tabs.is_empty() {
            return vec![Line::from("no chats")];
        }

        let mut spans = Vec::new();

        for (index, label) in self.tabs.iter().enumerate() {
            if index > 0 {
                spans.push(Span::raw("  "));
            }

            let style = if index == self.active_tab {
                Style::default()
                    .fg(dedicated_black_colour())
                    .bg(primary_colour())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(dedicated_grey_colour())
            };

            spans.push(Span::styled(format!(" {} ", label), style));
        }

        vec![Line::from(spans)]
    }
}

impl Widget for TabsBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let frame_block = Block::default()
            .border_style(Style::default().fg(secondary_colour()))
            .borders(Borders::ALL)
            .bg(secondary_colour());

        let content_area = frame_block.inner(area);
        frame_block.render(area, buf);

        Paragraph::new(self.tab_lines()).render(content_area, buf);
    }
}
