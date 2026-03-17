use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::settings::settings_keybinds::{AppKeybindsSettings, KeyCommand};
use crate::tui::util::secondary_colour;
use crate::{
    input::InputMode,
    tui::util::{dedicated_alt_mode_colour, dedicated_black_colour, dedicated_mode_colour},
};

pub fn render(
    area: Rect,
    buf: &mut Buffer,
    input_mode: InputMode,
    scroll_text: &str,
    keybinds: &AppKeybindsSettings,
) {
    let frame_block = Block::default()
        .border_style(Style::default().fg(secondary_colour()))
        .borders(Borders::ALL)
        .bg(secondary_colour());

    let content_area = frame_block.inner(area);
    frame_block.render(area, buf);

    let sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(16),
            Constraint::Min(1),
            Constraint::Length(24),
        ])
        .split(content_area);

    let (mode_label, mode_bg, center_text) = match input_mode {
        InputMode::Normal => (
            " normal ",
            dedicated_mode_colour(),
            format!(
                "{} insert | {}/{} switch | {} new | {} close | {}/{} scroll | {} quit",
                keybinds.label_for(KeyCommand::EnterInputMode),
                keybinds.label_for(KeyCommand::NextTab),
                keybinds.label_for(KeyCommand::PrevTab),
                keybinds.label_for(KeyCommand::NewChatTab),
                keybinds.label_for(KeyCommand::CloseCurrentTab),
                keybinds.label_for(KeyCommand::ScrollDown),
                keybinds.label_for(KeyCommand::ScrollUp),
                keybinds.label_for(KeyCommand::ExitApp),
            ),
        ),
        InputMode::Insert => (
            " insert ",
            dedicated_alt_mode_colour(),
            format!(
                "{} send | {} normal | {} delete | placeholder: chars and model hint",
                keybinds.label_for(KeyCommand::Submit),
                keybinds.label_for(KeyCommand::ExitInputMode),
                keybinds.label_for(KeyCommand::Backspace),
            ),
        ),
    };

    Paragraph::new(Line::from(vec![Span::styled(
        mode_label,
        Style::default()
            .bg(mode_bg)
            .fg(dedicated_black_colour())
            .bold(),
    )]))
    .render(sections[0], buf);

    Paragraph::new(center_text)
        .centered()
        .render(sections[1], buf);

    Paragraph::new(scroll_text)
        .right_aligned()
        .render(sections[2], buf);
}
