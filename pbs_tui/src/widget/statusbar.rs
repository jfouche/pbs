use crossterm::style::Color;

use super::{Buffer, Widget};

#[derive(Default)]
pub struct StatusBar {
    pub text: String,
}

impl Widget for StatusBar {
    type Action = ();
    fn display(&self, buf: &mut Buffer) {
        let line = format!("{}{}", self.text, " ".repeat(buf.width() - self.text.len()));
        buf.put_str(&line, 0, buf.height() - 2, Color::White, Color::Black);
    }
}
