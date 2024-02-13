use crossterm::style::Color;

use super::{BufferAccessor, Widget};

#[derive(Default)]
pub struct StatusBar {
    pub text: String,
}

impl Widget for StatusBar {
    fn display(&self, buf: &mut impl BufferAccessor) {
        let line = format!("{}{}", self.text, " ".repeat(buf.width() - self.text.len()));
        buf.put_str(&line, 0, 0, Color::White, Color::Black);
    }
}
