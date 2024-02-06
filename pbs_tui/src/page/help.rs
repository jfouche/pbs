use crate::widget::{Buffer, Paragraph, Title, Widget};

pub struct PageHelp {}

const HELP_TEXT: &str = r#"CTRL-h : this help page
CTRL-s : Search page
CTRL-x : Exit application"#;

impl PageHelp {
    pub fn new() -> Self {
        PageHelp {}
    }
}

impl Widget for PageHelp {
    fn display(&self, buf: &mut Buffer) {
        buf.add(Title("HELP".to_string()));
        buf.add(Paragraph(HELP_TEXT.to_string()));
    }
}
