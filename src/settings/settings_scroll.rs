//! scroll settings for transcript navigation behavior.

use serde::Deserialize;

/// configurable step sizes for chat transcript movement.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AppScrollSettings {
    /// wrapped-line step used by incremental up/down commands.
    pub line_step: usize,
    /// wrapped-line step used by page up/down commands.
    pub page_step: usize,
}

/// default scroll step sizes used when not configured.
impl Default for AppScrollSettings {
    fn default() -> Self {
        Self {
            line_step: 1,
            page_step: 10,
        }
    }
}