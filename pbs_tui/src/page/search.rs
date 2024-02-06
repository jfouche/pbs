use crate::{
    widget::{Buffer, Title, Widget},
    PbsAction,
};
use crossterm::event::Event;
use pbs_core::Item;

#[derive(Default)]
pub struct PageSearch {
    items: Vec<Item>,
}

impl PageSearch {
    pub fn set_items(&mut self, items: Vec<Item>) {
        self.items = items;
    }
}

impl Widget for PageSearch {
    type Action = PbsAction;

    fn display(&self, buf: &mut Buffer) {
        buf.add(Title("SEARCH".to_string()));

        for (i, item) in self.items.iter().enumerate() {
            buf.put_str(item.name(), 1, i + 3);
        }
    }

    fn handle_event(&mut self, _event: &Event) -> Option<Self::Action> {
        None
    }
}
