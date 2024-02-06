use super::{Buffer, Widget};

pub struct StatusBar;

impl Widget for StatusBar {
    fn display(&self, buf: &mut Buffer) {
        // let line = "═".repeat(buf.width() - 1);
        let line = "-".repeat(buf.width());
        buf.put_str(&line, 0, buf.height() - 2);
    }
}
