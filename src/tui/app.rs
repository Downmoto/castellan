//! app shell composition and event-facing api.
//! this module owns top-level layout and delegates chat behavior.

use crate::{
    input::InputMode,
    settings::prelude::settings,
    settings::settings_keybinds::KeyCommand,
    tui::{
        components::{
            chat::{ChatState, ChatWidget},
            info_sidebar::InfoSidebar,
            status_bar,
        },
        util::dedicated_black_colour,
    },
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::Rect,
    style::Style,
    widgets::{Block, Widget},
};

pub enum CommandResult {
    /// command was handled with no external side effect.
    None,
    /// app loop should terminate.
    Exit,
    /// caller should submit this message to the llm service.
    Submit { tab_index: usize, message: String },
}

struct ChatTabs {
    chats: Vec<ChatState>,
    active_tab: usize,
}

impl ChatTabs {
    fn with_first_tab() -> Self {
        let mut first_chat = ChatState::default();
        first_chat.set_title(ChatTabs::default_tab_title(1));

        Self {
            chats: vec![first_chat],
            active_tab: 0,
        }
    }

    fn default_tab_title(index: usize) -> String {
        format!("chat {}", index)
    }

    fn active_index(&self) -> usize {
        self.active_tab
    }

    fn active(&self) -> &ChatState {
        &self.chats[self.active_tab]
    }

    fn active_mut(&mut self) -> &mut ChatState {
        &mut self.chats[self.active_tab]
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut ChatState> {
        self.chats.iter_mut()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut ChatState> {
        self.chats.get_mut(index)
    }

    fn close_active(&mut self) {
        if self.chats.len() <= 1 {
            return;
        }

        self.chats.remove(self.active_tab);
        if self.active_tab >= self.chats.len() {
            self.active_tab = self.chats.len().saturating_sub(1);
        }
    }

    fn add_new_chat(&mut self) {
        let mut chat = ChatState::default();
        chat.set_title(ChatTabs::default_tab_title(self.chats.len() + 1));
        self.chats.push(chat);
        self.active_tab = self.chats.len().saturating_sub(1);
    }

    fn next(&mut self) {
        if self.chats.len() <= 1 {
            return;
        }

        self.active_tab = (self.active_tab + 1) % self.chats.len();
    }

    fn prev(&mut self) {
        if self.chats.len() <= 1 {
            return;
        }

        self.active_tab = (self.active_tab + self.chats.len() - 1) % self.chats.len();
    }

    fn labels(&self, rename_state: &RenameState) -> Vec<String> {
        self.chats
            .iter()
            .enumerate()
            .map(|(index, chat)| {
                if index == self.active_tab
                    && let RenameState::Active(draft) = rename_state
                {
                    return format!("{}█", draft);
                }

                chat.title().to_string()
            })
            .collect()
    }
}

enum RenameState {
    Inactive,
    Active(String),
}

impl RenameState {
    fn start(&mut self) -> bool {
        if matches!(self, Self::Active(_)) {
            return false;
        }

        *self = Self::Active(String::new());
        true
    }

    fn is_active(&self) -> bool {
        matches!(self, Self::Active(_))
    }

    fn push_char(&mut self, ch: char) {
        if let Self::Active(draft) = self {
            draft.push(ch);
        }
    }

    fn backspace(&mut self) {
        if let Self::Active(draft) = self {
            draft.pop();
        }
    }

    fn cancel(&mut self) {
        *self = Self::Inactive;
    }

    fn finish(&mut self) -> Option<String> {
        let Self::Active(draft) = std::mem::replace(self, Self::Inactive) else {
            return None;
        };

        let trimmed = draft.trim().to_string();
        if trimmed.is_empty() {
            return None;
        }

        Some(trimmed)
    }
}

/// root tui state for cross-component composition.
///
/// state invariants:
/// - `tabs` always contains at least one chat.
/// - `tabs.active_tab` always points to a valid chat index.
/// - rename mode is only entered while input mode is normal.
pub struct Castellan {
    tabs: ChatTabs,
    pub input_mode: InputMode,
    rename_state: RenameState,
}

impl Default for Castellan {
    fn default() -> Self {
        Self {
            tabs: ChatTabs::with_first_tab(),
            input_mode: InputMode::Normal,
            rename_state: RenameState::Inactive,
        }
    }
}

impl Castellan {
    /// splits the full terminal area into chat, sidebar, and status regions.
    fn layout_regions(area: Rect) -> (Rect, Rect, Rect) {
        let page = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .spacing(1)
            .split(area);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .flex(Flex::Center)
            .spacing(2)
            .constraints([Constraint::Percentage(100), Constraint::Length(30)])
            .split(page[0]);

        (columns[0], columns[1], page[1])
    }

    /// closes the active tab when more than one tab exists.
    ///
    /// if rename mode is active it is cancelled before closing.
    pub fn close_current_tab(&mut self) {
        self.cancel_current_tab_rename();
        self.tabs.close_active();
    }

    /// starts rename mode for the active tab.
    ///
    /// this is ignored outside normal mode to avoid input-mode conflicts.
    pub fn rename_current_tab(&mut self) {
        if self.input_mode != InputMode::Normal {
            return;
        }

        self.rename_state.start();
    }

    /// returns `true` when the active tab is currently in rename mode.
    pub fn is_renaming_current_tab(&self) -> bool {
        self.rename_state.is_active()
    }

    /// appends one character to the rename draft.
    pub fn rename_current_tab_push_char(&mut self, ch: char) {
        self.rename_state.push_char(ch);
    }

    /// removes one character from the rename draft.
    pub fn rename_current_tab_backspace(&mut self) {
        self.rename_state.backspace();
    }

    /// applies rename draft to the active tab title if non-empty.
    pub fn commit_current_tab_rename(&mut self) {
        let Some(new_title) = self.rename_state.finish() else {
            return;
        };

        self.tabs.active_mut().set_title(new_title);
    }

    /// exits rename mode and discards any draft content.
    pub fn cancel_current_tab_rename(&mut self) {
        self.rename_state.cancel();
    }

    /// updates chat viewport metrics from the current terminal area.
    ///
    /// call this before rendering when terminal size changes so wrapped-line
    /// scroll bounds remain correct.
    pub fn update_viewport_from_area(&mut self, area: Rect) {
        let (chat_area, _, _) = Self::layout_regions(area);

        for chat in self.tabs.iter_mut() {
            chat.set_viewport_from_area(chat_area);
        }
    }

    /// handles key input for rename mode and reports whether the key was consumed.
    pub fn handle_rename_key_event(&mut self, key_event: KeyEvent) -> bool {
        if !self.is_renaming_current_tab() {
            return false;
        }

        match key_event.code {
            KeyCode::Enter => self.commit_current_tab_rename(),
            KeyCode::Esc => self.cancel_current_tab_rename(),
            KeyCode::Backspace => self.rename_current_tab_backspace(),
            KeyCode::Char(ch)
                if !key_event.modifiers.contains(KeyModifiers::CONTROL)
                    && !key_event.modifiers.contains(KeyModifiers::ALT) =>
            {
                self.rename_current_tab_push_char(ch)
            }
            _ => {}
        }

        true
    }

    /// applies a resolved command and returns any loop-level side effect.
    ///
    /// dispatch contract:
    /// - mode and tab commands mutate local state only.
    /// - submit returns `CommandResult::Submit` with tab index and user message.
    /// - exit returns `CommandResult::Exit` for the outer event loop.
    pub fn apply_command(
        &mut self,
        command: KeyCommand,
        scroll_line_step: usize,
        scroll_page_step: usize,
    ) -> CommandResult {
        match command {
            KeyCommand::EnterInputMode => {
                self.input_mode = InputMode::Insert;
                CommandResult::None
            }
            KeyCommand::ExitInputMode => {
                self.input_mode = InputMode::Normal;
                CommandResult::None
            }
            KeyCommand::NewChatTab => {
                self.tabs.add_new_chat();
                CommandResult::None
            }
            KeyCommand::NextTab => {
                self.tabs.next();
                CommandResult::None
            }
            KeyCommand::PrevTab => {
                self.tabs.prev();
                CommandResult::None
            }
            KeyCommand::Backspace => {
                self.backspace();
                CommandResult::None
            }
            KeyCommand::CloseCurrentTab => {
                self.close_current_tab();
                CommandResult::None
            }
            KeyCommand::RenameCurrentTab => {
                self.rename_current_tab();
                CommandResult::None
            }
            KeyCommand::ScrollUp => {
                self.tabs.active_mut().scroll_up(scroll_line_step);
                CommandResult::None
            }
            KeyCommand::ScrollDown => {
                self.tabs.active_mut().scroll_down(scroll_line_step);
                CommandResult::None
            }
            KeyCommand::PageUp => {
                self.tabs.active_mut().scroll_up(scroll_page_step);
                CommandResult::None
            }
            KeyCommand::PageDown => {
                self.tabs.active_mut().scroll_down(scroll_page_step);
                CommandResult::None
            }
            KeyCommand::ScrollToBottom => {
                self.tabs.active_mut().scroll_to_bottom();
                CommandResult::None
            }
            KeyCommand::ExitApp => CommandResult::Exit,
            KeyCommand::Submit => {
                let Some((tab_index, message)) = self.take_input_for_submit() else {
                    return CommandResult::None;
                };

                self.push_user_message(message.clone());
                CommandResult::Submit { tab_index, message }
            }
        }
    }
}

impl Castellan {
    /// appends a typed character to the chat input buffer.
    pub fn push_char(&mut self, ch: char) {
        self.tabs.active_mut().push_char(ch);
    }

    /// removes the last character from the chat input buffer.
    pub fn backspace(&mut self) {
        self.tabs.active_mut().backspace();
    }

    /// returns a trimmed message if submit is valid and clears input.
    pub fn take_input_for_submit(&mut self) -> Option<(usize, String)> {
        let active_tab = self.tabs.active_index();
        self.tabs
            .active_mut()
            .take_input_for_submit()
            .map(|message| (active_tab, message))
    }

    /// appends a user-authored message to the transcript.
    pub fn push_user_message(&mut self, content: String) {
        self.tabs.active_mut().push_user_message(content);
    }

    /// appends an assistant-authored message to the transcript.
    pub fn push_assistant_message_for_tab(&mut self, tab_index: usize, content: String) {
        if let Some(chat) = self.tabs.get_mut(tab_index) {
            chat.push_assistant_message(content);
        }
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

        let (chat_area, sidebar_area, status_area) = Castellan::layout_regions(area);

        ChatWidget::new(self.tabs.active()).render(chat_area, buf);

        InfoSidebar::new(
            &self.tabs.labels(&self.rename_state),
            self.tabs.active_index(),
            self.is_renaming_current_tab(),
        )
        .render(sidebar_area, buf);

        status_bar::render(
            status_area,
            buf,
            self.input_mode,
            &self.tabs.active().status_text(),
            &settings().keybinds,
        );
    }
}
