//! composable tui widgets used by the app shell.
//!
//! render order in the page:
//! - `chat`: transcript and input panel.
//! - `info_sidebar`: tab list and auxiliary status.
//! - `status_bar`: mode and command hints.

/// chat transcript state and rendering widget.
pub mod chat;
/// sidebar with tab list and mode-specific hints.
pub mod info_sidebar;
/// user-authored transcript message rendering helpers.
pub mod request_message;
/// assistant-authored transcript message rendering helpers.
pub mod response_message;
/// bottom status bar renderer.
pub mod status_bar;
/// bottom input box renderer.
pub mod user_input;
