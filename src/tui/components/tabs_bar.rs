use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

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
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
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
        Paragraph::new(self.tab_lines())
            .block(Block::default().title("tabs").borders(Borders::ALL))
            .render(area, buf);
    }
}
