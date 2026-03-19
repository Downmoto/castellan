//! shared tui styling tokens and layout helper utilities.

use ratatui::style::Color;

/// accent color for active ui elements and emphasis.
pub fn primary_colour() -> Color {
    Color::Yellow
}

/// default panel background color for framed components.
pub fn secondary_colour() -> Color {
    Color::Rgb(30, 30, 30)
}

/// dedicated page background color.
pub fn dedicated_black_colour() -> Color {
    Color::Black
}

/// muted neutral foreground for low-priority text.
pub fn dedicated_grey_colour() -> Color {
    Color::Gray
}

/// darker muted foreground for separators and metadata.
pub fn dedicated_dark_grey_colour() -> Color {
    Color::DarkGray
}

/// background color used by normal-mode status badge.
pub fn dedicated_mode_colour() -> Color {
    Color::Rgb(255, 105, 180)
}

/// background color used by insert-mode status badge.
pub fn dedicated_alt_mode_colour() -> Color {
    Color::White
}

/// background color used by the input panel.
pub fn dedicated_input_background_colour() -> Color {
    Color::Rgb(33, 85, 99)
}

/// counts visual rows after wrapping text to a fixed character width.
///
/// behavior notes:
/// - width `0` returns `0` rows.
/// - empty lines still consume one visual row.
/// - newline-separated lines are wrapped independently.
pub fn wrapped_line_count(text: &str, width: usize) -> usize {
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
