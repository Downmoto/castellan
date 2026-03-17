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
};

/// root tui state for cross-component composition.
pub struct Castellan {
    chats: Vec<ChatState>,
    active_tab: usize,
    input_mode: InputMode,
    tab_rename_buffer: Option<String>,
}

impl Default for Castellan {
    fn default() -> Self {
        let mut first_chat = ChatState::default();
        first_chat.set_title(Self::default_tab_title(1));

        Self {
            chats: vec![first_chat],
            active_tab: 0,
            input_mode: InputMode::Normal,
            tab_rename_buffer: None,
        }
    }
}

impl Castellan {
    fn default_tab_title(index: usize) -> String {
        format!("chat {}", index)
    }

    fn active_chat(&self) -> &ChatState {
        &self.chats[self.active_tab]
    }

    fn active_chat_mut(&mut self) -> &mut ChatState {
        &mut self.chats[self.active_tab]
    }

    fn tab_labels(&self) -> Vec<String> {
        self.chats
            .iter()
            .enumerate()
            .map(|(index, chat)| {
                if index == self.active_tab && let Some(draft) = &self.tab_rename_buffer {
                    return format!("{}█", draft);
                }

                chat.title().to_string()
            })
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
        self.cancel_current_tab_rename();
        self.input_mode = InputMode::Insert;
    }

    /// exits input mode and returns to command-only mode.
    pub fn exit_input_mode(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn close_current_tab(&mut self) {
        if self.chats.len() <= 1 {
            return;
        }

        self.cancel_current_tab_rename();
        self.chats.remove(self.active_tab);
        if self.active_tab >= self.chats.len() {
            self.active_tab = self.chats.len().saturating_sub(1);
        }
    }

    pub fn rename_current_tab(&mut self) {
        if self.input_mode != InputMode::Normal || self.tab_rename_buffer.is_some() {
            return;
        }

        self.tab_rename_buffer = Some(String::new());
    }

    pub fn is_renaming_current_tab(&self) -> bool {
        self.tab_rename_buffer.is_some()
    }

    pub fn rename_current_tab_push_char(&mut self, ch: char) {
        if let Some(draft) = &mut self.tab_rename_buffer {
            draft.push(ch);
        }
    }

    pub fn rename_current_tab_backspace(&mut self) {
        if let Some(draft) = &mut self.tab_rename_buffer {
            draft.pop();
        }
    }

    pub fn commit_current_tab_rename(&mut self) {
        let Some(draft) = self.tab_rename_buffer.take() else {
            return;
        };

        let new_title = draft.trim();
        if new_title.is_empty() {
            return;
        }

        self.active_chat_mut().set_title(new_title.to_string());
    }

    pub fn cancel_current_tab_rename(&mut self) {
        self.tab_rename_buffer = None;
    }

    /// creates a new chat tab and focuses it.
    pub fn new_chat_tab(&mut self) {
        let mut chat = ChatState::default();
        chat.set_title(Self::default_tab_title(self.chats.len() + 1));
        self.chats.push(chat);
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
            ])
            .flex(Flex::Start)
            .spacing(1)
            .split(area);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .flex(Flex::Center)
            .spacing(2)
            .constraints([Constraint::Percentage(100), Constraint::Length(40)])
            .split(page[0]);

        ChatWidget::new(self.active_chat()).render(columns[0], buf);

        InfoSidebar::new(
            &self.tab_labels(),
            self.active_tab,
            self.is_renaming_current_tab(),
        )
        .render(columns[1], buf);
        status_bar::render(
            page[1],
            buf,
            self.input_mode,
            &self.active_chat().status_text(),
            settings().keybinds(),
        );
    }
}
