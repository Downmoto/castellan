//! crate entry point for castellan runtime modules.

/// input translation and mode-aware key command resolution.
pub mod input;
/// tracing setup and console event formatting.
pub mod logging;
/// llm request execution and assistant reply generation.
pub mod llm;
/// application settings loading and typed configuration models.
pub mod settings;
/// terminal ui state, components, and lifecycle helpers.
pub mod tui;