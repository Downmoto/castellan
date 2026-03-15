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

        let transcript = if self.messages.is_empty() {
            "no messages yet. type and press enter to send.".to_string()
        } else {
            self.messages
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
        let clamped_scroll_from_bottom = self.scroll_from_bottom.min(max_scroll_from_bottom);
        let top_scroll = total_lines
            .saturating_sub(transcript_height.saturating_add(clamped_scroll_from_bottom));
        let top_scroll = top_scroll.min(u16::MAX as usize) as u16;

        paragraph.scroll((top_scroll, 0)).render(sections[0], buf);

        let input_with_cursor = format!("> {}█", self.input);

        Paragraph::new(input_with_cursor).render(sections[1], buf);
    }
}
