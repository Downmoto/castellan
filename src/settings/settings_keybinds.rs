//! Keybinding configuration and key-event to command mapping.
//!
//! This module defines:
//! - the complete set of application key commands,
//! - configurable key chord settings deserialized from config,
//! - default bindings used when config is omitted,
//! - runtime resolution from `KeyEvent` to `KeyCommand`.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Deserializer};

/// Canonical list of input commands understood by the application.
///
/// These commands are mode-filtered by higher-level input handling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyCommand {
    /// Exit the application.
    ExitApp,
    /// Switch from normal mode to input mode.
    EnterInputMode,
    /// Leave input mode and return to normal mode.
    ExitInputMode,
    /// Create a new chat tab.
    NewChatTab,
    /// Select the next tab.
    NextTab,
    /// Select the previous tab.
    PrevTab,
    /// Delete one character in the active input field.
    Backspace,
    /// Submit the active input.
    Submit,
    /// Scroll chat/content upward.
    ScrollUp,
    /// Scroll chat/content downward.
    ScrollDown,
    /// Scroll up by one page.
    PageUp,
    /// Scroll down by one page.
    PageDown,
    /// Jump scrolling to the bottom of content.
    ScrollToBottom,
    /// Close the currently selected tab.
    CloseCurrentTab,
    /// Rename the currently selectd tab
    RenameCurrentTab

}

impl KeyCommand {
    /// Stable command resolution order used by [`AppKeybindsSettings::resolve_command`].
    ///
    /// Ordering matters if multiple configured chords overlap; the first
    /// matching command wins.
    const ORDERED: [Self; 15] = [
        Self::ExitApp,
        Self::EnterInputMode,
        Self::ExitInputMode,
        Self::NewChatTab,
        Self::NextTab,
        Self::PrevTab,
        Self::Backspace,
        Self::Submit,
        Self::ScrollUp,
        Self::ScrollDown,
        Self::PageUp,
        Self::PageDown,
        Self::ScrollToBottom,
        Self::CloseCurrentTab,
        Self::RenameCurrentTab
    ];
}

#[derive(Clone, Debug, Deserialize)]
pub struct KeyChordSettings {
    /// Required key code portion of the chord.
    #[serde(deserialize_with = "deserialize_key_code")]
    code: KeyCode,
    /// Whether `Ctrl` must be present.
    #[serde(default)]
    ctrl: bool,
    /// Whether `Alt` must be present.
    #[serde(default)]
    alt: bool,
    /// Whether `Shift` must be present.
    #[serde(default)]
    shift: bool,
}

impl KeyChordSettings {
    /// Returns `true` when the incoming key event satisfies this key chord.
    ///
    /// Character codes are compared case-insensitively so configured `j` can
    /// still match a shifted `J` key event when `shift = true`.
    ///
    /// Modifiers are matched exactly. This allows pairs like `j` and
    /// `Shift+j` to coexist without overlapping.
    pub fn matches(&self, key_event: &KeyEvent) -> bool {
        let (expected_code, expected_shift) = normalize_tab_code(self.code, self.shift);
        let (actual_code, actual_shift) = normalize_tab_code(
            key_event.code,
            key_event.modifiers.contains(KeyModifiers::SHIFT),
        );

        if !code_matches(expected_code, actual_code) {
            return false;
        }

        if key_event.modifiers.contains(KeyModifiers::CONTROL) != self.ctrl {
            return false;
        }

        if key_event.modifiers.contains(KeyModifiers::ALT) != self.alt {
            return false;
        }

        if actual_shift != expected_shift {
            return false;
        }

        true
    }

    /// returns a human-readable label for the configured key chord.
    pub fn label(&self) -> String {
        let mut parts: Vec<&str> = Vec::new();

        if self.ctrl {
            parts.push("ctrl");
        }

        if self.alt {
            parts.push("alt");
        }

        if self.shift {
            parts.push("shift");
        }

        let key_label = key_code_label(self.code);
        parts.push(&key_label);
        parts.join("+")
    }
}

fn normalize_tab_code(code: KeyCode, shift: bool) -> (KeyCode, bool) {
    match code {
        KeyCode::BackTab => (KeyCode::Tab, true),
        _ => (code, shift),
    }
}

fn code_matches(expected: KeyCode, actual: KeyCode) -> bool {
    match (expected, actual) {
        (KeyCode::Char(expected), KeyCode::Char(actual)) => {
            expected.eq_ignore_ascii_case(&actual)
        }
        _ => expected == actual,
    }
}

impl Default for KeyChordSettings {
    /// Provides a conservative fallback key chord (`Esc`).
    fn default() -> Self {
        key(KeyCode::Esc)
    }
}

/// User-configurable keybinding collection for all supported commands.
///
/// The `#[serde(default)]` annotation means missing fields in configuration are
/// filled from [`Default`], enabling partial overrides.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AppKeybindsSettings {
    /// Binding for [`KeyCommand::ExitApp`].
    pub exit_app: KeyChordSettings,
    /// Binding for [`KeyCommand::EnterInputMode`].
    pub enter_input_mode: KeyChordSettings,
    /// Binding for [`KeyCommand::ExitInputMode`].
    pub exit_input_mode: KeyChordSettings,
    /// Binding for [`KeyCommand::NewChatTab`].
    pub new_chat_tab: KeyChordSettings,
    /// Binding for [`KeyCommand::NextTab`].
    pub next_tab: KeyChordSettings,
    /// Binding for [`KeyCommand::PrevTab`].
    pub prev_tab: KeyChordSettings,
    /// Binding for [`KeyCommand::Backspace`].
    pub backspace: KeyChordSettings,
    /// Binding for [`KeyCommand::Submit`].
    pub submit: KeyChordSettings,
    /// Binding for [`KeyCommand::ScrollUp`].
    pub scroll_up: KeyChordSettings,
    /// Binding for [`KeyCommand::ScrollDown`].
    pub scroll_down: KeyChordSettings,
    /// Binding for [`KeyCommand::PageUp`].
    pub page_up: KeyChordSettings,
    /// Binding for [`KeyCommand::PageDown`].
    pub page_down: KeyChordSettings,
    /// Binding for [`KeyCommand::ScrollToBottom`].
    pub scroll_to_bottom: KeyChordSettings,
    /// Binding for [`KeyCommand::CloseCurrentTab`].
    pub close_current_tab: KeyChordSettings,
    /// Binding for [`KeyCommand::RenameCurrentTab`].
    pub rename_current_tab: KeyChordSettings
}

impl Default for AppKeybindsSettings {
    /// Built-in Vim/TUI-friendly keybinding defaults.
    fn default() -> Self {
        Self {
            exit_app: ctrl_key('c'),
            enter_input_mode: key(KeyCode::Char('i')),
            exit_input_mode: key(KeyCode::Esc),
            new_chat_tab: ctrl_key('t'),
            next_tab: key(KeyCode::Tab),
            prev_tab: key(KeyCode::BackTab),
            backspace: key(KeyCode::Backspace),
            submit: key(KeyCode::Enter),
            scroll_up: key(KeyCode::Char('k')),
            scroll_down: key(KeyCode::Char('j')),
            page_up: shift_key('k'),
            page_down: shift_key('j'),
            scroll_to_bottom: key(KeyCode::End),
            close_current_tab: ctrl_key('w'),
            rename_current_tab: ctrl_key('r')
        }
    }
}

impl AppKeybindsSettings {
    /// Resolves a raw key event to the first matching command, if any.
    ///
    /// Matching is performed in [`KeyCommand::ORDERED`] sequence.
    pub fn resolve_command(&self, key_event: &KeyEvent) -> Option<KeyCommand> {
        KeyCommand::ORDERED
            .into_iter()
            .find(|&command| self.binding_for(command).matches(key_event))
    }

    /// returns the display label for a command's current binding.
    pub fn label_for(&self, command: KeyCommand) -> String {
        self.binding_for(command).label()
    }

    /// Returns the configured chord associated with a given command.
    fn binding_for(&self, command: KeyCommand) -> &KeyChordSettings {
        match command {
            KeyCommand::ExitApp => &self.exit_app,
            KeyCommand::EnterInputMode => &self.enter_input_mode,
            KeyCommand::ExitInputMode => &self.exit_input_mode,
            KeyCommand::NewChatTab => &self.new_chat_tab,
            KeyCommand::NextTab => &self.next_tab,
            KeyCommand::PrevTab => &self.prev_tab,
            KeyCommand::Backspace => &self.backspace,
            KeyCommand::Submit => &self.submit,
            KeyCommand::ScrollUp => &self.scroll_up,
            KeyCommand::ScrollDown => &self.scroll_down,
            KeyCommand::PageUp => &self.page_up,
            KeyCommand::PageDown => &self.page_down,
            KeyCommand::ScrollToBottom => &self.scroll_to_bottom,
            KeyCommand::CloseCurrentTab => &self.close_current_tab,
            KeyCommand::RenameCurrentTab => &self.rename_current_tab
        }
    }
}

fn key_code_label(code: KeyCode) -> String {
    match code {
        KeyCode::Enter => "enter".to_string(),
        KeyCode::Tab => "tab".to_string(),
        KeyCode::BackTab => "backtab".to_string(),
        KeyCode::Backspace => "backspace".to_string(),
        KeyCode::Up => "up".to_string(),
        KeyCode::Down => "down".to_string(),
        KeyCode::PageUp => "pageup".to_string(),
        KeyCode::PageDown => "pagedown".to_string(),
        KeyCode::End => "end".to_string(),
        KeyCode::Esc => "esc".to_string(),
        KeyCode::Char(ch) => ch.to_ascii_lowercase().to_string(),
        _ => "key".to_string(),
    }
}

/// Creates an unmodified key chord for a single key code.
fn key(code: KeyCode) -> KeyChordSettings {
    KeyChordSettings {
        code,
        ctrl: false,
        alt: false,
        shift: false,
    }
}

/// Creates a `Ctrl+<char>` key chord.
fn ctrl_key(ch: char) -> KeyChordSettings {
    KeyChordSettings {
        code: KeyCode::Char(ch),
        ctrl: true,
        alt: false,
        shift: false,
    }
}

/// Creates a `Shift+<char>` key chord.
fn shift_key(ch: char) -> KeyChordSettings {
    KeyChordSettings {
        code: KeyCode::Char(ch),
        ctrl: false,
        alt: false,
        shift: true,
    }
}

/// Deserializes a textual key code from configuration into `crossterm` form.
fn deserialize_key_code<'de, D>(deserializer: D) -> Result<KeyCode, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    parse_key_code(&value).ok_or_else(|| {
        serde::de::Error::custom(format!(
            "invalid key code '{value}', expected one of: single character, enter, tab, backtab, backspace, up, down, pageup, pagedown, end, esc"
        ))
    })
}

/// Parses normalized key strings (for example `enter`, `pgdn`, `a`).
fn parse_key_code(value: &str) -> Option<KeyCode> {
    let lowered = value.trim().to_ascii_lowercase();

    match lowered.as_str() {
        "enter" => Some(KeyCode::Enter),
        "tab" => Some(KeyCode::Tab),
        "backtab" | "shift+tab" => Some(KeyCode::BackTab),
        "backspace" => Some(KeyCode::Backspace),
        "up" => Some(KeyCode::Up),
        "down" => Some(KeyCode::Down),
        "pageup" | "pgup" => Some(KeyCode::PageUp),
        "pagedown" | "pgdown" | "pgdn" => Some(KeyCode::PageDown),
        "end" => Some(KeyCode::End),
        "esc" | "escape" => Some(KeyCode::Esc),
        _ => {
            let mut chars = lowered.chars();
            match (chars.next(), chars.next()) {
                (Some(ch), None) => Some(KeyCode::Char(ch)),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AppKeybindsSettings, KeyCommand};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn shift_char_binding_matches_uppercase_char_event() {
        let keybinds = AppKeybindsSettings {
            page_down: super::shift_key('j'),
            ..AppKeybindsSettings::default()
        };

        let command = keybinds.resolve_command(&KeyEvent::new(
            KeyCode::Char('J'),
            KeyModifiers::SHIFT,
        ));

        assert_eq!(command, Some(KeyCommand::PageDown));
    }

    #[test]
    fn unshifted_and_shifted_char_bindings_do_not_overlap() {
        let keybinds = AppKeybindsSettings {
            scroll_down: super::key(KeyCode::Char('j')),
            page_down: super::shift_key('j'),
            ..AppKeybindsSettings::default()
        };

        let plain = keybinds.resolve_command(&KeyEvent::new(
            KeyCode::Char('j'),
            KeyModifiers::NONE,
        ));
        let shifted = keybinds.resolve_command(&KeyEvent::new(
            KeyCode::Char('J'),
            KeyModifiers::SHIFT,
        ));

        assert_eq!(plain, Some(KeyCommand::ScrollDown));
        assert_eq!(shifted, Some(KeyCommand::PageDown));
    }

    #[test]
    fn prev_tab_matches_backtab_with_shift_modifier() {
        let keybinds = AppKeybindsSettings::default();

        let command = keybinds.resolve_command(&KeyEvent::new(
            KeyCode::BackTab,
            KeyModifiers::SHIFT,
        ));

        assert_eq!(command, Some(KeyCommand::PrevTab));
    }

    #[test]
    fn prev_tab_matches_shift_tab_encoding() {
        let keybinds = AppKeybindsSettings::default();

        let command = keybinds.resolve_command(&KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT));

        assert_eq!(command, Some(KeyCommand::PrevTab));
    }
}
