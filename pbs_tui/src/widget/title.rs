use crossterm::style::Color;

use super::{Buffer, Widget};

/// Display a title centered at the top of the screen.
pub struct Title(pub String);

impl Widget for Title {
    fn display(&self, screen: &mut Buffer) {
        screen.put_str(
            &self.0,
            screen.width() / 2 - self.0.len() / 2,
            0,
            Color::Black,
            Color::White,
        );
    }
}
