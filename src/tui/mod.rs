//! TUI surface modules and terminal lifecycle helpers.

/// root app state and command application logic.
pub mod app;
/// reusable widgets used by the app shell.
pub mod components;
mod util;

/// Convenience imports and setup helpers for terminal-backed UI sessions.
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

    /// Concrete terminal type used by the application.
    type TuiTerminal = Terminal<CrosstermBackend<std::io::Stdout>>;

    /// Initializes terminal state for interactive TUI rendering.
    ///
    /// This enables raw mode, switches to the alternate screen, and turns on
    /// mouse capture before constructing a `ratatui` terminal instance.
    pub fn init_terminal() -> Result<TuiTerminal, Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        execute!(stdout, EnableMouseCapture)?;
        
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(terminal)
    }

    /// Restores terminal state after the TUI exits.
    ///
    /// This should be called exactly once for every successful
    /// [`init_terminal`] call to avoid leaving the shell in raw/alternate mode.
    pub fn deinit_terminal(terminal: &mut TuiTerminal) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), DisableMouseCapture)?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}
