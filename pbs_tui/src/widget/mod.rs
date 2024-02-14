mod paragraph;
mod prompt;
mod statusbar;
mod title;

pub use paragraph::Paragraph;
pub use prompt::{Prompt, PromptEvent};
pub use statusbar::StatusBar;
pub use title::Title;

use crate::buffer::BufferAccessor;

pub trait Widget {
    fn display(&self, buf: &mut impl BufferAccessor);
}

impl<T: Widget> Widget for &mut T {
    fn display(&self, buf: &mut impl BufferAccessor) {
        (**self).display(buf);
    }
}
