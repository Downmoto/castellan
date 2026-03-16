use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AppScrollSettings {
    pub line_step: usize,
    pub page_step: usize,
}

impl Default for AppScrollSettings {
    fn default() -> Self {
        Self {
            line_step: 1,
            page_step: 10,
        }
    }
}