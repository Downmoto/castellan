//! chat state and rendering primitives.
//! this module owns transcript, input, and chat-local scrolling logic.

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
        util::{dedicated_dark_grey_colour, primary_colour, wrapped_line_count},
    },
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    prelude::{Buffer, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget, Wrap},
};

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

    fn total_lines_for_width(&self, width: usize) -> usize {
        if self.messages.is_empty() {
            return 1;
        }

        let width = width.max(1);
        self.messages
            .iter()
            .enumerate()
            .map(|(index, message)| {
                let message_lines = message_plain_rows(message, width)
                    .iter()
                    .map(|row| wrapped_line_count(row, width))
                    .sum::<usize>();
                let separator = usize::from(index + 1 < self.messages.len());
                message_lines + separator
            })
            .sum()
    }
}

/// mutable chat domain state used by app events and rendering.
///
/// invariants:
/// - `scroll_from_bottom` is measured in wrapped transcript rows.
/// - `scroll_from_bottom` is always clamped to valid transcript bounds.
/// - viewport dimensions are updated through `set_viewport_from_area`.
pub struct ChatState {
    title: String,
    input: String,
    transcript: ChatTranscript,
    scroll_from_bottom: usize,
    transcript_viewport_width: usize,
    transcript_viewport_height: usize,
}

impl Default for ChatState {
    fn default() -> Self {
        Self {
            title: "chat".to_string(),
            input: String::new(),
            transcript: ChatTranscript::default(),
            scroll_from_bottom: 0,
            transcript_viewport_width: 0,
            transcript_viewport_height: 0,
        }
    }
}

impl ChatState {
    fn input_height_for_area(&self, area: Rect) -> u16 {
        calculate_input_height(&self.input, area)
    }

    fn content_width_for_area(area: Rect) -> usize {
        area.width.saturating_sub(2).max(1) as usize
    }

    fn transcript_height_for_area(area: Rect, input_height: u16) -> usize {
        area.height.saturating_sub(input_height).max(1) as usize
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    /// updates the display title shown in sidebar tab labels.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// updates viewport-derived values used for wrapped-line scrolling.
    ///
    /// call this whenever the chat render area changes size.
    pub fn set_viewport_from_area(&mut self, area: Rect) {
        let input_height = self.input_height_for_area(area);
        let content_width = Self::content_width_for_area(area);
        let transcript_height = Self::transcript_height_for_area(area, input_height);

        self.transcript_viewport_width = content_width;
        self.transcript_viewport_height = transcript_height;
        self.clamp_scroll();
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
        let next = self.scroll_from_bottom.saturating_add(lines);
        self.scroll_from_bottom = next.min(self.max_scroll_from_bottom());
    }

    /// moves transcript scroll down by wrapped lines.
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_from_bottom = self.scroll_from_bottom.saturating_sub(lines);
        self.clamp_scroll();
    }

    /// jumps transcript scroll to latest content.
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_from_bottom = 0;
    }

    /// builds status text consumed by the app status bar.
    pub fn status_text(&self) -> String {
        let max_scroll = self.max_scroll_from_bottom();
        if self.scroll_from_bottom == 0 {
            "scroll: bottom".to_string()
        } else if self.scroll_from_bottom >= max_scroll {
            "scroll: top".to_string()
        } else {
            format!("scroll: {} lines up", self.scroll_from_bottom)
        }
    }

    /// computes total wrapped transcript lines for current viewport width.
    fn total_transcript_lines(&self) -> usize {
        self.transcript
            .total_lines_for_width(self.transcript_viewport_width.max(1))
    }

    /// computes total wrapped transcript lines for an explicit width.
    fn total_transcript_lines_for_width(&self, width: usize) -> usize {
        self.transcript.total_lines_for_width(width)
    }

    /// computes how far the viewport can scroll upward from bottom.
    fn max_scroll_from_bottom(&self) -> usize {
        let viewport = self.transcript_viewport_height.max(1);
        self.total_transcript_lines().saturating_sub(viewport)
    }

    /// clamps scroll offset so it stays within valid transcript bounds.
    fn clamp_scroll(&mut self) {
        self.scroll_from_bottom = self.scroll_from_bottom.min(self.max_scroll_from_bottom());
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

/// generates plain rows for wrapped-line counting and scroll bounds math.
fn message_plain_rows(message: &ChatMessage, width: usize) -> Vec<String> {
    with_message_widget(
        message,
        width,
        |widget| widget.plain_rows(),
        |widget| widget.plain_rows(),
    )
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
    state: &'a ChatState,
}

impl<'a> ChatWidget<'a> {
    /// creates a chat widget bound to the provided state.
    pub fn new(state: &'a ChatState) -> Self {
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

            let transcript_height = sections[0].height as usize;
            let paragraph = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
            let total_lines = self
                .state
                .total_transcript_lines_for_width(transcript_width);
            let max_scroll_from_bottom = total_lines.saturating_sub(transcript_height);
            let clamped_scroll_from_bottom =
                self.state.scroll_from_bottom.min(max_scroll_from_bottom);
            let top_scroll = total_lines
                .saturating_sub(transcript_height.saturating_add(clamped_scroll_from_bottom));
            let top_scroll = top_scroll.min(u16::MAX as usize) as u16;

            paragraph.scroll((top_scroll, 0)).render(sections[0], buf);
        }

        UserInputWidget::new(&self.state.input).render(sections[1], buf);
    }
}
