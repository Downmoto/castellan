use ratatui::style::Color;

pub fn primary_colour() -> Color {
    Color::Yellow
}

pub fn secondary_colour() -> Color {
    Color::Rgb(30, 30, 30)
}

pub fn dedicated_black_colour() -> Color {
    Color::Black
}

pub fn dedicated_grey_colour() -> Color {
    Color::Gray
}

pub fn dedicated_dark_grey_colour() -> Color {
    Color::DarkGray
}

pub fn dedicated_mode_colour() -> Color {
    Color::Rgb(255, 105, 180)
}

pub fn dedicated_alt_mode_colour() -> Color {
    Color::White
}

pub fn dedicated_input_background_colour() -> Color {
    Color::Rgb(33, 85, 99)
}

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
