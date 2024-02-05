use crossterm::event::Event;

use crate::screen::{Screen, Title};

pub struct PageSeach {
    pub pattern: String,
}

impl PageSeach {
    pub fn new() -> Self {
        PageSeach {
            pattern: "".to_string(),
        }
    }

    pub fn display(&mut self, screen: &mut Screen) {
        screen.add(Title("SEARCH".to_string()));
        screen.put_str("Pattern", 1, 3);
    }

    pub fn handle_event(&mut self, event: Event) {}
}
