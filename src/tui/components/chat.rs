//! chat state and rendering primitives.
//! this module owns transcript, input, and chat-local scrolling logic.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

#[derive(Clone, Debug)]
/// single row in the transcript with speaker and content.
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
}

#[derive(Default)]
/// mutable chat domain state used by app events and rendering.
pub struct ChatState {
    input: String,
    messages: Vec<ChatMessage>,
    scroll_from_bottom: usize,
    transcript_viewport_width: usize,
    transcript_viewport_height: usize,
}

impl ChatState {
    /// updates viewport-derived values used for wrapped-line scrolling.
    pub fn set_viewport_from_area(&mut self, area: Rect) {
        let content_width = area.width.saturating_sub(2).max(1) as usize;
        let content_height = area.height.saturating_sub(2);
        let transcript_height = content_height.saturating_sub(1).max(1) as usize;

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
        self.messages.push(ChatMessage {
            sender: "you".to_string(),
            content,
        });
        self.scroll_to_bottom();
    }

    /// appends an assistant message and anchors transcript to bottom.
    pub fn push_assistant_message(&mut self, content: String) {
        self.messages.push(ChatMessage {
            sender: "assistant".to_string(),
            content,
        });
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
        let scroll = if self.scroll_from_bottom == 0 {
            "scroll: bottom".to_string()
        } else if self.scroll_from_bottom >= max_scroll {
            "scroll: top".to_string()
        } else {
            format!("scroll: {} lines up", self.scroll_from_bottom)
        };

        format!(
            "{} | keys: up/down pgup/pgdn end | submit: enter | quit: esc/ctrl+c",
            scroll
        )
    }

    /// computes total wrapped transcript lines for current viewport width.
    fn total_transcript_lines(&self) -> usize {
        if self.messages.is_empty() {
            return 1;
        }

        let width = self.transcript_viewport_width.max(1);
        self.messages
            .iter()
            .map(|message| {
                let row = format!("{}: {}", message.sender, message.content);
                wrapped_line_count(&row, width)
            })
            .sum()
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

/// counts wrapped visual lines for a string at a fixed width.
fn wrapped_line_count(text: &str, width: usize) -> usize {
    if width == 0 {
        return 0;
    }

    text.split('\n')
        .map(|line| {
            let visual_width = line.chars().count();
            let cells = visual_width.max(1);
            (cells - 1) / width + 1
        })
        .sum()
}

impl Widget for ChatWidget<'_> {
    /// renders chat transcript and input row inside a bordered frame.
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let frame_block = Block::default().title("chat").borders(Borders::ALL);
        let content_area = frame_block.inner(area);
        frame_block.render(area, buf);

        let sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(content_area);

        let transcript = if self.state.messages.is_empty() {
            "no messages yet. type and press enter to send.".to_string()
        } else {
            self.state
                .messages
                .iter()
                .map(|message| format!("{}: {}", message.sender, message.content))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let transcript_height = sections[0].height as usize;
        let transcript_width = sections[0].width.max(1);
        let paragraph = Paragraph::new(transcript.clone()).wrap(Wrap { trim: false });
        let total_lines = wrapped_line_count(&transcript, transcript_width as usize);
        let max_scroll_from_bottom = total_lines.saturating_sub(transcript_height);
        let clamped_scroll_from_bottom = self
            .state
            .scroll_from_bottom
            .min(max_scroll_from_bottom);
        let top_scroll = total_lines
            .saturating_sub(transcript_height.saturating_add(clamped_scroll_from_bottom));
        let top_scroll = top_scroll.min(u16::MAX as usize) as u16;

        paragraph.scroll((top_scroll, 0)).render(sections[0], buf);

        let input_with_cursor = format!("> {}█", self.state.input);

        Paragraph::new(input_with_cursor).render(sections[1], buf);
    }
}
