use tui::style::{Color, Modifier, Style};

pub struct RuscodeStyle {}

impl RuscodeStyle {
    pub fn unfocus_mode() -> Style {
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM)
    }
    pub fn focus_mode() -> Style {
        Style::default().fg(Color::White)
    }
}
