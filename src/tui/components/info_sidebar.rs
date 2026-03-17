//! sidebar component for auxiliary info and session tab navigation.
//! this widget renders two stacked frames:
//! - a top info area for metadata/status content.
//! - a bottom sessions area that lists chat tabs and rename hints.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

use crate::tui::util::{dedicated_black_colour, dedicated_grey_colour, dedicated_input_background_colour, primary_colour, secondary_colour};

/// renders the right-side panel containing info and session tabs.
///
/// the tabs slice is borrowed from app state, and `active_tab` points to the
/// currently selected index in that slice.
pub struct InfoSidebar<'a> {
    tabs: &'a [String],
    active_tab: usize,
    is_renaming: bool,
}

impl<'a> InfoSidebar<'a> {
    /// builds a sidebar view model for the current frame.
    pub fn new(tabs: &'a [String], active_tab: usize, is_renaming: bool) -> Self {
        Self {
            tabs,
            active_tab,
            is_renaming,
        }
    }

    /// truncates a tab label to fit a target character width.
    ///
    /// when truncation is needed and room allows, this appends `...`.
    fn truncate_label(label: &str, max_chars: usize) -> String {
        if max_chars == 0 {
            return String::new();
        }

        let total_chars = label.chars().count();
        if total_chars <= max_chars {
            return label.to_string();
        }

        if max_chars <= 3 {
            return ".".repeat(max_chars);
        }

        let prefix_len = max_chars - 3;
        let prefix: String = label.chars().take(prefix_len).collect();
        format!("{}...", prefix)
    }

    /// renders one tab row, including active marker and active background fill.
    ///
    /// active rows keep the marker segment styled separately and pad the label
    /// segment so highlight starts at text and reaches the right edge.
    fn tab_line(&self, label: &str, index: usize, max_width: usize) -> Line<'static> {
        if max_width == 0 {
            return Line::from("");
        }

        let is_active = index == self.active_tab;
        let marker_width = 2;
        let label_max_width = max_width.saturating_sub(marker_width);
        let trimmed_label = Self::truncate_label(label, label_max_width);

        let label_style = if is_active {
            Style::default()
                .fg(dedicated_black_colour())
                .bg(primary_colour())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(dedicated_grey_colour())
        };

        let marker_style = if is_active {
            Style::default()
                .fg(primary_colour())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(dedicated_grey_colour())
        };

        let marker = if is_active { ">" } else { " " };

        if is_active {
            let mut label_cell = trimmed_label;
            let label_cell_width = label_cell.chars().count();
            if label_cell_width < label_max_width {
                label_cell.push_str(&" ".repeat(label_max_width - label_cell_width));
            }

            return Line::from(vec![
                Span::styled(format!("{} ", marker), marker_style),
                Span::styled(label_cell, label_style),
            ]);
        }

        Line::from(vec![
            Span::styled(format!("{} ", marker), marker_style),
            Span::styled(trimmed_label, label_style),
        ])
    }

    /// builds the visible tab lines constrained by the sessions frame viewport.
    ///
    /// this keeps the active tab in view, adds overflow indicators, and reserves
    /// optional rows for rename-mode hints.
    fn tab_lines(&self, max_rows: usize, max_width: usize) -> Vec<Line<'static>> {
        if max_rows == 0 {
            return Vec::new();
        }

        let mut lines = Vec::new();
        let rename_hint_rows = if self.is_renaming { 3 } else { 0 };
        let rows_for_tabs = max_rows.saturating_sub(rename_hint_rows);

        if self.tabs.is_empty() {
            if rows_for_tabs > 0 {
                lines.push(Line::from("no chats"));
            }
        } else if rows_for_tabs > 0 {
            let tab_count = self.tabs.len();
            let mut visible_slots = rows_for_tabs.min(tab_count);

            let (window_start, show_top_overflow, show_bottom_overflow) = loop {
                let start = self
                    .active_tab
                    .saturating_sub(visible_slots / 2)
                    .min(tab_count.saturating_sub(visible_slots));
                let show_top = start > 0;
                let show_bottom = start + visible_slots < tab_count;
                let required_rows = visible_slots + usize::from(show_top) + usize::from(show_bottom);

                if required_rows <= rows_for_tabs || visible_slots == 1 {
                    break (start, show_top, show_bottom);
                }

                visible_slots -= 1;
            };

            let indicator_style = Style::default().fg(dedicated_grey_colour());

            if show_top_overflow {
                lines.push(Line::from(vec![Span::styled("^ more", indicator_style)]));
            }

            let window_end = window_start + visible_slots;
            for (index, label) in self.tabs[window_start..window_end].iter().enumerate() {
                lines.push(self.tab_line(label, window_start + index, max_width));
            }

            if show_bottom_overflow {
                lines.push(Line::from(vec![Span::styled("v more", indicator_style)]));
            }
        }

        if self.is_renaming {
            if lines.len() < max_rows {
                lines.push(Line::from(vec![Span::styled(
                    "renaming session:",
                    Style::default().fg(primary_colour()).add_modifier(Modifier::BOLD),
                )]));
            }

            if lines.len() < max_rows {
                lines.push(Line::from("enter submit"));
            }

            if lines.len() < max_rows {
                lines.push(Line::from("esc cancel"));
            }
        }

        lines
    }

}

impl Widget for InfoSidebar<'_> {
    /// renders the full sidebar as two vertical sections.
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized 
    {
        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let info_block = Block::default()
            .border_style(Style::default().fg(secondary_colour()))
            .borders(Borders::ALL)
            .bg(secondary_colour());

        let info_area = info_block.inner(sections[0]);
        info_block.render(sections[0], buf);

        Paragraph::new("metadata / useful info").render(info_area, buf);


        let tabs_block = Block::default()
            .title(" sessions ")
            .title_style(Style::default().white().bg(dedicated_input_background_colour()))
            .border_style(Style::default().fg(secondary_colour()))
            .borders(Borders::ALL)
            .padding(Padding::vertical(1))
            .bg(secondary_colour());

        let tabs_area = tabs_block.inner(sections[1]);
        tabs_block.render(sections[1], buf);

        let max_rows = tabs_area.height as usize;
        let max_width = tabs_area.width as usize;
        Paragraph::new(self.tab_lines(max_rows, max_width)).render(tabs_area, buf);
    }
}