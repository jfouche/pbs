use crossterm::style::Color;

use super::{BufferAccessor, Widget};

/// Display a title centered at the top of the screen.
pub struct Title(pub String);

impl Widget for Title {
    fn display(&self, buf: &mut impl BufferAccessor) {
        buf.put_str(
            &self.0,
            buf.width() / 2 - self.0.len() / 2,
            0,
            Color::Black,
            Color::White,
        );
    }
}
