//! app shell composition and event-facing api.
//! this module owns top-level layout and delegates chat behavior.

use crate::{
    input::InputMode,
    settings::prelude::settings,
    tui::{components::info_sidebar::InfoSidebar, util::dedicated_black_colour},
};

use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::Rect,
    style::{Style},
    widgets::{Block, Widget},
};

use super::components::{
    chat::{ChatState, ChatWidget},
    status_bar,
    tabs_bar::TabsBar,
};

/// root tui state for cross-component composition.
pub struct Castellan {
    chats: Vec<ChatState>,
    active_tab: usize,
    input_mode: InputMode,
}

impl Default for Castellan {
    fn default() -> Self {
        Self {
            chats: vec![ChatState::default()],
            active_tab: 0,
            input_mode: InputMode::Normal,
        }
    }
}

impl Castellan {
    fn active_chat(&self) -> &ChatState {
        &self.chats[self.active_tab]
    }

    fn active_chat_mut(&mut self) -> &mut ChatState {
        &mut self.chats[self.active_tab]
    }

    fn tab_labels(&self) -> Vec<String> {
        (1..=self.chats.len())
            .map(|index| format!("chat {}", index))
            .collect()
    }

    /// returns the current active chat index.
    pub fn active_tab_index(&self) -> usize {
        self.active_tab
    }

    /// returns the current global input mode.
    pub fn input_mode(&self) -> InputMode {
        self.input_mode
    }

    /// enters input mode for text entry.
    pub fn enter_input_mode(&mut self) {
        self.input_mode = InputMode::Input;
    }

    /// exits input mode and returns to command-only mode.
    pub fn exit_input_mode(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn close_current_tab(&mut self) {
        if self.chats.len() <= 1 {
            return;
        }

        self.chats.remove(self.active_tab);
        if self.active_tab >= self.chats.len() {
            self.active_tab = self.chats.len().saturating_sub(1);
        }
    }

    /// creates a new chat tab and focuses it.
    pub fn new_chat_tab(&mut self) {
        self.chats.push(ChatState::default());
        self.active_tab = self.chats.len().saturating_sub(1);
    }

    /// switches focus to the next chat tab.
    pub fn next_tab(&mut self) {
        if self.chats.len() <= 1 {
            return;
        }

        self.active_tab = (self.active_tab + 1) % self.chats.len();
    }

    /// switches focus to the previous chat tab.
    pub fn prev_tab(&mut self) {
        if self.chats.len() <= 1 {
            return;
        }

        self.active_tab = (self.active_tab + self.chats.len() - 1) % self.chats.len();
    }

    /// updates chat viewport metrics from the current terminal area.
    pub fn update_viewport_from_area(&mut self, area: Rect) {
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

        for chat in &mut self.chats {
            chat.set_viewport_from_area(columns[0]);
        }
    }
}

impl Castellan {
    /// appends a typed character to the chat input buffer.
    pub fn push_char(&mut self, ch: char) {
        self.active_chat_mut().push_char(ch);
    }

    /// removes the last character from the chat input buffer.
    pub fn backspace(&mut self) {
        self.active_chat_mut().backspace();
    }

    /// returns a trimmed message if submit is valid and clears input.
    pub fn take_input_for_submit(&mut self) -> Option<(usize, String)> {
        self.active_chat_mut()
            .take_input_for_submit()
            .map(|message| (self.active_tab, message))
    }

    /// appends a user-authored message to the transcript.
    pub fn push_user_message(&mut self, content: String) {
        self.active_chat_mut().push_user_message(content);
    }

    /// appends an assistant-authored message to the transcript.
    pub fn push_assistant_message_for_tab(&mut self, tab_index: usize, content: String) {
        if let Some(chat) = self.chats.get_mut(tab_index) {
            chat.push_assistant_message(content);
        }
    }

    /// scrolls transcript upward by the requested number of wrapped lines.
    pub fn scroll_up(&mut self, lines: usize) {
        self.active_chat_mut().scroll_up(lines);
    }

    /// scrolls transcript downward by the requested number of wrapped lines.
    pub fn scroll_down(&mut self, lines: usize) {
        self.active_chat_mut().scroll_down(lines);
    }

    /// resets transcript scroll to the newest message.
    pub fn scroll_to_bottom(&mut self) {
        self.active_chat_mut().scroll_to_bottom();
    }

}

impl Widget for &Castellan {
    /// renders the full app page and delegates chat drawing.
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Block::default()
            .style(Style::default().bg(dedicated_black_colour()))
            .render(area, buf);

        let page = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .flex(Flex::Start)
            .spacing(1)
            .split(area);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .flex(Flex::Center)
            .spacing(2)
            .constraints([Constraint::Percentage(100), Constraint::Length(25)])
            .split(page[0]);

        ChatWidget::new(self.active_chat()).render(columns[0], buf);

        InfoSidebar::new().render(columns[1], buf);
        TabsBar::new(&self.tab_labels(), self.active_tab).render(page[1], buf);
        status_bar::render(
            page[2],
            buf,
            self.input_mode,
            &self.active_chat().status_text(),
            settings().keybinds(),
        );
    }
}
