use crate::{
    widget::{Buffer, Title, Widget},
    PbsAction,
};
use crossterm::{
    event::{Event, KeyCode},
    style::Color,
};
use pbs_core::Item;

#[derive(Default)]
pub struct PageSearch {
    sel: usize,
    items: Vec<Item>,
}

impl PageSearch {
    pub fn set_items(&mut self, items: Vec<Item>) {
        self.items = items;
    }
    pub fn handle_event(&mut self, event: &Event) -> Option<PbsAction> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Up => {}
                KeyCode::Down => {}
                KeyCode::Enter => return Some(PbsAction::ViewItem(1)),
                _ => {}
            },
            _ => {}
        }
        None
    }
}

impl Widget for PageSearch {
    fn display(&self, buf: &mut Buffer) {
        buf.add(Title("SEARCH".to_string()));

        for (i, item) in self.items.iter().enumerate() {
            buf.put_str(item.name(), 1, i + 3, Color::Black, Color::White);
        }
    }
}
