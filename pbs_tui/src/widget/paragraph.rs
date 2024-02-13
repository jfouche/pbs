use crossterm::style::Color;

use super::{Buffer, Widget};

pub struct Paragraph(pub String);

impl Widget for Paragraph {
    fn display(&self, screen: &mut Buffer) {
        for (i, line) in self.0.lines().enumerate() {
            screen.put_str(line, 0, i + 3, Color::Black, Color::White);
        }
    }
}
