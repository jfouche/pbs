use crossterm::event::Event;

use crate::widget::{Buffer, Title, Widget};

pub struct PageSeach {
    pub pattern: String,
}

impl PageSeach {
    pub fn new() -> Self {
        PageSeach {
            pattern: "".to_string(),
        }
    }
}

impl Widget for PageSeach {
    fn display(&self, buf: &mut Buffer) {
        buf.add(Title("SEARCH".to_string()));
        buf.put_str("Pattern", 1, 3);
    }

    fn handle_event(&mut self, _event: &Event) {}
}
