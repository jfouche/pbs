use crossterm::event::Event;

use crate::screen::{Paragraph, Screen, Title};

pub struct PageHelp {}

const HELP_TEXT: &str = r#"CTRL-h : this help page
CTRL-s : Search page"#;

impl PageHelp {
    pub fn new() -> Self {
        PageHelp {}
    }

    pub fn display(&mut self, screen: &mut Screen) {
        screen.add(Title("HELP".to_string()));
        screen.add(Paragraph(HELP_TEXT.to_string()));
    }

    pub fn handle_event(&mut self, event: Event) {}
}
