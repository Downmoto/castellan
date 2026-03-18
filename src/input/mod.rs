//! Input mode and key resolution utilities for the TUI.
//!
//! This module maps low-level `crossterm` key events into higher-level actions
//! that the application can execute. Resolution is mode-aware so that
//! navigation shortcuts in normal mode do not interfere with text entry in
//! input mode.

use crate::settings::settings_keybinds::AppKeybindsSettings;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub use crate::settings::settings_keybinds::KeyCommand;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum InputMode {
    /// Command/navigation mode where keybinds trigger app commands.
    #[default]
    Normal,
    /// Text-entry mode where printable keys are inserted into the input buffer.
    Insert,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyAction {
    /// Execute an app-level command.
    Command(KeyCommand),
    /// Insert a printable character into the active text input.
    InsertChar(char),
    /// Ignore the key event.
    Noop,
}

/// Resolves `KeyEvent` values into mode-specific [`KeyAction`] values.
///
/// The resolver is intentionally strict in `InputMode::Input`: it allows only
/// editing/submit commands and plain character insertion, while ignoring most
/// control-key chords.
pub struct KeybindResolver<'a> {
    keybinds: &'a AppKeybindsSettings,
}

impl<'a> KeybindResolver<'a> {
    /// Creates a resolver backed by the provided keybinding configuration.
    pub fn new(keybinds: &'a AppKeybindsSettings) -> Self {
        Self { keybinds }
    }

    /// Maps a raw key event into a high-level action for the active input mode.
    ///
    /// Resolution rules:
    /// - `InputMode::Normal`: only navigation/app management commands are
    ///   emitted; all other keys become [`KeyAction::Noop`].
    /// - `InputMode::Input`: exit/edit/submit commands are allowed; plain
    ///   printable characters become [`KeyAction::InsertChar`].
    /// - `Ctrl` or `Alt` modified characters are ignored while typing to avoid
    ///   accidental command execution from text-entry mode.
    pub fn resolve(&self, key_event: KeyEvent, mode: InputMode) -> KeyAction {
        match mode {
            InputMode::Normal => {
                if let Some(command) = self.keybinds.resolve_command(&key_event) {
                    match command {
                        KeyCommand::ExitApp
                        | KeyCommand::EnterInputMode
                        | KeyCommand::NewChatTab
                        | KeyCommand::NextTab
                        | KeyCommand::PrevTab
                        | KeyCommand::ScrollUp
                        | KeyCommand::ScrollDown
                        | KeyCommand::PageUp
                        | KeyCommand::PageDown
                        | KeyCommand::ScrollToBottom
                        | KeyCommand::CloseCurrentTab
                        | KeyCommand::RenameCurrentTab => return KeyAction::Command(command),
                        _ => {}
                    }
                }

                KeyAction::Noop
            }
            InputMode::Insert => {
                if let Some(command) = self.keybinds.resolve_command(&key_event) {
                    match command {
                        KeyCommand::ExitInputMode
                        | KeyCommand::Backspace
                        | KeyCommand::NextTab
                        | KeyCommand::PrevTab
                        | KeyCommand::ExitApp
                        | KeyCommand::Submit => {
                            return KeyAction::Command(command);
                        }
                        _ => {}
                    }
                }

                if self.keybinds.backspace.matches(&key_event) {
                    return KeyAction::Command(KeyCommand::Backspace);
                }

                if self.keybinds.submit.matches(&key_event) {
                    return KeyAction::Command(KeyCommand::Submit);
                }

                if let KeyCode::Char(ch) = key_event.code {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL)
                        || key_event.modifiers.contains(KeyModifiers::ALT)
                    {
                        return KeyAction::Noop;
                    }

                    return KeyAction::InsertChar(ch);
                }

                KeyAction::Noop
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InputMode, KeyAction, KeyCommand, KeybindResolver};
    use crate::settings::settings_keybinds::AppKeybindsSettings;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn i_enters_input_mode_from_normal() {
        let keybinds = AppKeybindsSettings::default();
        let resolver = KeybindResolver::new(&keybinds);

        let action = resolver.resolve(
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            InputMode::Normal,
        );

        assert_eq!(action, KeyAction::Command(KeyCommand::EnterInputMode));
    }

    #[test]
    fn i_types_in_input_mode() {
        let keybinds = AppKeybindsSettings::default();
        let resolver = KeybindResolver::new(&keybinds);

        let action = resolver.resolve(
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            InputMode::Insert,
        );

        assert_eq!(action, KeyAction::InsertChar('i'));
    }

    #[test]
    fn close_tab_shortcut_is_ignored_in_input_mode() {
        let keybinds = AppKeybindsSettings::default();
        let resolver = KeybindResolver::new(&keybinds);

        let action = resolver.resolve(
            KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CONTROL),
            InputMode::Insert,
        );

        assert_eq!(action, KeyAction::Noop);
    }

    #[test]
    fn rename_tab_shortcut_is_allowed_in_normal_mode() {
        let keybinds = AppKeybindsSettings::default();
        let resolver = KeybindResolver::new(&keybinds);

        let action = resolver.resolve(
            KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
            InputMode::Normal,
        );

        assert_eq!(action, KeyAction::Command(KeyCommand::RenameCurrentTab));
    }

    #[test]
    fn rename_tab_shortcut_is_ignored_in_input_mode() {
        let keybinds = AppKeybindsSettings::default();
        let resolver = KeybindResolver::new(&keybinds);

        let action = resolver.resolve(
            KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
            InputMode::Insert,
        );

        assert_eq!(action, KeyAction::Noop);
    }
}
