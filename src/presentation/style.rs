use tui::style::{Color, Modifier, Style};

pub struct RuscodeStyle {}

impl RuscodeStyle {
    pub fn unfocus_mode() -> Style {
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM)
    }
    pub fn focus_mode() -> Style {
        Style::default().fg(Color::Rgb(122, 171, 212))
        // Style::default().fg(Color::White)
    }
    pub fn success() -> Style {
        Style::default().fg(Color::Rgb(169, 211, 171))
    }

    pub fn default_font() -> Style {
        Style::default().fg(Color::White)
    }
}
