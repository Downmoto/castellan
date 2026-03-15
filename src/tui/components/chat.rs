use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Buffer, Rect},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub sender: String,
    pub content: String,
}

pub struct ChatWidget<'a> {
    input: &'a str,
    messages: &'a [ChatMessage],
    scroll_from_bottom: usize,
}

impl<'a> ChatWidget<'a> {
    pub fn new(input: &'a str, messages: &'a [ChatMessage], scroll_from_bottom: usize) -> Self {
        Self {
            input,
            messages,
            scroll_from_bottom,
        }
    }
}

impl Widget for ChatWidget<'_> {
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

        let transcript_lines = if self.messages.is_empty() {
            vec!["no messages yet. type and press enter to send.".to_string()]
        } else {
            self.messages
                .iter()
                .map(|message| format!("{}: {}", message.sender, message.content))
                .collect::<Vec<_>>()
        };

        let transcript_height = sections[0].height as usize;
        let max_scroll_from_bottom = transcript_lines.len().saturating_sub(transcript_height);
        let clamped_scroll_from_bottom = self.scroll_from_bottom.min(max_scroll_from_bottom);

        let start = transcript_lines
            .len()
            .saturating_sub(transcript_height.saturating_add(clamped_scroll_from_bottom));
        let end = transcript_lines
            .len()
            .saturating_sub(clamped_scroll_from_bottom);

        let visible_transcript = transcript_lines[start..end].join("\n");

        Paragraph::new(visible_transcript)
            .wrap(Wrap { trim: false })
            .render(sections[0], buf);

        Paragraph::new(format!("> {}", self.input))
            .render(sections[1], buf);
    }
}
