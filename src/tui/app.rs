use ratatui::{style::{Modifier}, text::{Line, Span}, widgets::Widget};


#[derive(Default)]
pub struct Castellan {

}

impl Widget for &Castellan {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        TestWidget::new("Kevin".to_string()).render(area, buf);
        
    }
}

struct TestWidget {
    name: String
}

impl TestWidget {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

impl Widget for TestWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
        let hello = Span::raw("Hello, ");
        let name = Span::styled(self.name, Modifier::BOLD);
        let line = Line::from(vec![hello, name]);
        line.render(area, buf);
    }
}