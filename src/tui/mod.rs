pub mod app;

pub mod prelude {
    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    };
    use ratatui::{
        Terminal,
        backend::CrosstermBackend,
    };

    type TuiTerminal = Terminal<CrosstermBackend<std::io::Stdout>>;

    pub fn init_terminal() -> Result<TuiTerminal, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        execute!(stdout, EnableMouseCapture)?;
        
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(terminal)
    }

    pub fn deinit_terminal(terminal: &mut TuiTerminal) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), DisableMouseCapture)?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}
