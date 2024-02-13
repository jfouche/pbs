use crate::widget::{Buffer, Paragraph, Title, Widget};

pub struct PageHelp;

const HELP_TEXT: &str = r#"CTRL-h : this help page
CTRL-s : Search page
CTRL-n : Create a new MAKE item
CTRL-b : Create a new BUY item
CTRL-x : Exit application"#;

impl Widget for PageHelp {
    fn display(&self, buf: &mut Buffer) {
        buf.add(Title("HELP".to_string()));
        buf.add(Paragraph(HELP_TEXT.to_string()));
    }
}
