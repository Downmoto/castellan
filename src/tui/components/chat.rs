//! chat state and rendering primitives.
//! this module owns transcript, input, and chat-local scroll view state.

use crate::{
    settings::{
        prelude::settings,
        settings_keybinds::{AppKeybindsSettings, KeyCommand},
    },
    tui::{
        components::{
            request_message::RequestMessageWidget,
            response_message::ResponseMessageWidget,
            user_input::UserInputWidget,
        },
        util::{dedicated_dark_grey_colour, primary_colour},
    },
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Size},
    prelude::{Buffer, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, StatefulWidget, Widget, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

const CASTELLAN_ASCII: &str = r#"                                             
                           ▄▄ ▄▄             
                   █▄       ██ ██            
                  ▄██▄      ██ ██       ▄    
 ▄███▀ ▄▀▀█▄ ▄██▀█ ██ ▄█▀█▄ ██ ██ ▄▀▀█▄ ████▄
 ██    ▄█▀██ ▀███▄ ██ ██▄█▀ ██ ██ ▄█▀██ ██ ██
▄▀███▄▄▀█▄███▄▄██▀▄██▄▀█▄▄▄▄██▄██▄▀█▄██▄██ ▀█"#;

#[derive(Clone, Debug)]
/// speaker identity for a transcript message.
pub enum ChatSender {
    /// message authored by the local user.
    User,
    /// message authored by the assistant backend.
    Assistant,
}

#[derive(Clone, Debug)]
/// single row in the transcript with speaker and content.
pub struct ChatMessage {
    pub sender: ChatSender,
    pub content: String,
}

#[derive(Default)]
struct ChatTranscript {
    messages: Vec<ChatMessage>,
}

impl ChatTranscript {
    fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    fn iter(&self) -> std::slice::Iter<'_, ChatMessage> {
        self.messages.iter()
    }

    fn len(&self) -> usize {
        self.messages.len()
    }

    fn push_user_message(&mut self, content: String) {
        self.messages.push(ChatMessage {
            sender: ChatSender::User,
            content,
        });
    }

    fn push_assistant_message(&mut self, content: String) {
        self.messages.push(ChatMessage {
            sender: ChatSender::Assistant,
            content,
        });
    }

}

/// mutable chat domain state used by app events and rendering.
///
/// invariants:
/// - `scroll_state.offset().y` is clamped to the current transcript bounds.
/// - viewport dimensions are updated through `set_viewport_from_area` or render.
pub struct ChatState {
    title: String,
    input: String,
    transcript: ChatTranscript,
    scroll_state: ScrollViewState,
    transcript_content_height: u16,
    transcript_viewport_height: u16,
}

impl Default for ChatState {
    fn default() -> Self {
        Self {
            title: "chat".to_string(),
            input: String::new(),
            transcript: ChatTranscript::default(),
            scroll_state: ScrollViewState::new(),
            transcript_content_height: 1,
            transcript_viewport_height: 1,
        }
    }
}

impl ChatState {
    fn input_height_for_area(&self, area: Rect) -> u16 {
        calculate_input_height(&self.input, area)
    }

    fn transcript_height_for_area(area: Rect, input_height: u16) -> usize {
        area.height.saturating_sub(input_height).max(1) as usize
    }

    fn max_scroll_offset(&self) -> u16 {
        self.transcript_content_height
            .saturating_sub(self.transcript_viewport_height.saturating_sub(1))
    }

    fn clamp_scroll_offset(&mut self) {
        let mut offset = self.scroll_state.offset();
        offset.y = offset.y.min(self.max_scroll_offset());
        self.scroll_state.set_offset(offset);
    }

    fn set_transcript_metrics(&mut self, content_height: u16, viewport_height: u16) {
        self.transcript_content_height = content_height.max(1);
        self.transcript_viewport_height = viewport_height.max(1);
        self.clamp_scroll_offset();
    }

    fn set_scroll_offset_y(&mut self, y: u16) {
        let mut offset = self.scroll_state.offset();
        offset.y = y;
        self.scroll_state.set_offset(offset);
        self.clamp_scroll_offset();
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    /// updates the display title shown in sidebar tab labels.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// updates viewport-derived values used for transcript scrolling.
    ///
    /// call this whenever the chat render area changes size.
    pub fn set_viewport_from_area(&mut self, area: Rect) {
        let input_height = self.input_height_for_area(area);
        let transcript_height = Self::transcript_height_for_area(area, input_height) as u16;
        self.transcript_viewport_height = transcript_height.max(1);
    }

    /// appends a typed character to input.
    pub fn push_char(&mut self, ch: char) {
        self.input.push(ch);
    }

    /// removes one character from input.
    pub fn backspace(&mut self) {
        self.input.pop();
    }

    /// returns a submit-ready message and clears input when valid.
    pub fn take_input_for_submit(&mut self) -> Option<String> {
        let candidate = self.input.trim().to_string();
        if candidate.is_empty() {
            return None;
        }

        self.input.clear();
        Some(candidate)
    }

    /// appends a user message and anchors transcript to bottom.
    pub fn push_user_message(&mut self, content: String) {
        self.transcript.push_user_message(content);
        self.scroll_to_bottom();
    }

    /// appends an assistant message and anchors transcript to bottom.
    pub fn push_assistant_message(&mut self, content: String) {
        self.transcript.push_assistant_message(content);
        self.scroll_to_bottom();
    }

    /// moves transcript scroll up by wrapped lines.
    pub fn scroll_up(&mut self, lines: usize) {
        let step = lines.min(u16::MAX as usize) as u16;
        let y = self.scroll_state.offset().y.saturating_sub(step);
        self.set_scroll_offset_y(y);
    }

    /// moves transcript scroll down by wrapped lines.
    pub fn scroll_down(&mut self, lines: usize) {
        let step = lines.min(u16::MAX as usize) as u16;
        let y = self.scroll_state.offset().y.saturating_add(step);
        self.set_scroll_offset_y(y);
    }

    /// jumps transcript scroll to latest content.
    pub fn scroll_to_bottom(&mut self) {
        // Use a large offset and let render-time clamping anchor to true bottom.
        // This avoids stale pre-render content metrics pulling us off bottom.
        let mut offset = self.scroll_state.offset();
        offset.y = u16::MAX;
        self.scroll_state.set_offset(offset);
    }

    /// builds status text consumed by the app status bar.
    pub fn status_text(&self) -> String {
        let offset = self.scroll_state.offset().y;
        let max_scroll = self.max_scroll_offset();

        if offset >= max_scroll {
            "scroll: bottom".to_string()
        } else if offset == 0 {
            "scroll: top".to_string()
        } else {
            format!("scroll: {} lines up", max_scroll.saturating_sub(offset))
        }
    }
}

fn calculate_input_height(input: &str, area: Rect) -> u16 {
    let input_panel_width = area.width.saturating_sub(2);
    let mut input_height = UserInputWidget::required_height(input, input_panel_width);
    let max_input_height = area.height.saturating_sub(1).max(1);
    input_height = input_height.min(max_input_height);
    input_height
}

/// dispatches per-sender message adapters while sharing call-site logic.
fn with_message_widget<T>(
    message: &ChatMessage,
    width: usize,
    request: impl FnOnce(RequestMessageWidget<'_>) -> T,
    response: impl FnOnce(ResponseMessageWidget<'_>) -> T,
) -> T {
    match message.sender {
        ChatSender::User => request(RequestMessageWidget::new(&message.content, width)),
        ChatSender::Assistant => response(ResponseMessageWidget::new(&message.content, width)),
    }
}

/// generates styled rows for transcript rendering.
fn message_styled_rows(message: &ChatMessage, width: usize) -> Vec<Line<'static>> {
    with_message_widget(
        message,
        width,
        |widget| widget.styled_rows(),
        |widget| widget.styled_rows(),
    )
}

/// builds keybind rows shown in the empty-state help section.
fn empty_state_shortcuts(keybinds: &AppKeybindsSettings) -> Vec<(String, &'static str)> {
    vec![
        (keybinds.label_for(KeyCommand::NewChatTab), "new tab"),
        (keybinds.label_for(KeyCommand::CloseCurrentTab), "close tab"),
        (
            keybinds.label_for(KeyCommand::EnterInputMode),
            "insert mode",
        ),
        (keybinds.label_for(KeyCommand::ExitInputMode), "normal mode"),
    ]
}

/// builds centered empty-state lines for a brand + shortcuts screen.
fn empty_state_lines() -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    for ascii_line in CASTELLAN_ASCII.lines() {
        lines.push(Line::styled(
            ascii_line,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    }

    lines.push(Line::raw(""));
    lines.push(Line::styled(
        "Keyboard Shortcuts",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::White),
    ));
    lines.push(Line::styled(
        "─".repeat(18),
        Style::default().fg(dedicated_dark_grey_colour()),
    ));

    let shortcuts = empty_state_shortcuts(&settings().keybinds);
    let max_key_width = shortcuts
        .iter()
        .map(|(key, _)| key.chars().count())
        .max()
        .unwrap_or(0);
    let max_desc_width = shortcuts
        .iter()
        .map(|(_, desc)| desc.chars().count())
        .max()
        .unwrap_or(0);

    for (key, desc) in shortcuts {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{key:<max_key_width$}  "),
                Style::default()
                    .fg(primary_colour())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{desc:<max_desc_width$}"),
                Style::default().fg(dedicated_dark_grey_colour()),
            ),
        ]));
    }

    lines
}

/// chat renderer that reads from shared chat state.
pub struct ChatWidget<'a> {
    state: &'a mut ChatState,
}

impl<'a> ChatWidget<'a> {
    /// creates a chat widget bound to the provided state.
    pub fn new(state: &'a mut ChatState) -> Self {
        Self { state }
    }
}

impl Widget for ChatWidget<'_> {
    /// renders chat transcript and input row inside a bordered frame.
    ///
    /// rendering flow:
    /// - show empty-state content when transcript has no messages.
    /// - otherwise render all message rows and apply vertical scroll offset.
    /// - always render input widget in the bottom section.
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let input_height = calculate_input_height(&self.state.input, area);

        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(input_height)])
            .spacing(1)
            .split(area.inner(Margin {
                horizontal: 1,
                vertical: 0,
            }));

        if self.state.transcript.is_empty() {
            self.state
                .set_transcript_metrics(1, sections[0].height.max(1));

            let empty_lines = empty_state_lines();
            let empty_height = empty_lines.len();
            let available_height = sections[0].height as usize;

            let (top_gap, bottom_gap) = if empty_height >= available_height {
                (0, 0)
            } else {
                let gap = (available_height - empty_height) / 2;
                (gap, available_height - empty_height - gap)
            };

            let empty_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(top_gap as u16),
                    Constraint::Length(empty_height as u16),
                    Constraint::Length(bottom_gap as u16),
                ])
                .split(sections[0]);

            let paragraph = Paragraph::new(Text::from(empty_lines))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });
            paragraph.render(empty_layout[1], buf);
        } else {
            let mut lines: Vec<Line<'static>> = Vec::new();
            let transcript_width = sections[0].width.max(1) as usize;

            for (index, message) in self.state.transcript.iter().enumerate() {
                lines.extend(message_styled_rows(message, transcript_width));
                if index + 1 < self.state.transcript.len() {
                    lines.push(Line::raw(""));
                }
            }

            let content_height = lines.len().max(1).min(u16::MAX as usize) as u16;
            self.state
                .set_transcript_metrics(content_height, sections[0].height.max(1));

            let size = Size::new(sections[0].width.max(1), content_height);
            let mut scroll_view = ScrollView::new(size)
                .scrollbars_visibility(ScrollbarVisibility::Never);
            let content_area = Rect::new(0, 0, size.width, size.height);

            scroll_view.render_widget(Paragraph::new(Text::from(lines)), content_area);
            scroll_view.render(sections[0], buf, &mut self.state.scroll_state);
        }

        UserInputWidget::new(&self.state.input).render(sections[1], buf);
    }
}
