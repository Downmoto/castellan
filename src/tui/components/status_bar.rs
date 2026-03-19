//! bottom status bar renderer for mode hints and scroll status.

use crate::{
    input::InputMode,
    settings::settings_keybinds::{AppKeybindsSettings, KeyCommand},
    tui::util::{
        dedicated_alt_mode_colour, 
        dedicated_black_colour, 
        dedicated_mode_colour, 
    },
};

use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    prelude::{Buffer, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// renders mode badge, command hint text, and transcript scroll status.
///
/// parameter roles:
/// - `input_mode` selects mode badge colors and command hint set.
/// - `scroll_text` is precomputed from chat state and right-aligned.
/// - `keybinds` provides user-configured key labels for command hints.
pub fn render(
    area: Rect,
    buf: &mut Buffer,
    input_mode: InputMode,
    scroll_text: &str,
    keybinds: &AppKeybindsSettings,
) {
    let sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(16),
            Constraint::Min(1),
            Constraint::Length(24),
        ])
        .split(area.inner(Margin { horizontal: 1, vertical: 0 }));

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
