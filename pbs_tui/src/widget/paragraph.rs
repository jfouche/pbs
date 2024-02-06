use super::{Buffer, Widget};

pub struct Paragraph(pub String);

impl Widget for Paragraph {
    type Action = ();
    fn display(&self, screen: &mut Buffer) {
        for (i, line) in self.0.lines().enumerate() {
            screen.put_str(line, 0, i + 3);
        }
    }
}
