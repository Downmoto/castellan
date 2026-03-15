use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Widget,
};

use super::components::{chat, info_sidebar, status_bar, tabs_bar};

#[derive(Default)]
pub struct Castellan;

impl Widget for &Castellan {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where Self: Sized {
        let page = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(area);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Length(30)])
            .split(page[0]);

        chat::render(columns[0], buf);
        info_sidebar::render(columns[1], buf);
        tabs_bar::render(page[1], buf);
        status_bar::render(page[2], buf);

    }
}
