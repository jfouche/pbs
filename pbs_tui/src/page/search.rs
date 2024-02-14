use crate::{
    buffer::BufferAccessor,
    widget::{Title, Widget},
    PbsAction,
};
use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
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
        self.sel = 0;
    }
    pub fn handle_event(&mut self, event: &Event) -> Option<PbsAction> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Up => {
                    if self.sel > 0 {
                        self.sel -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.sel + 1 < self.items.len() {
                        self.sel += 1;
                    }
                }
                KeyCode::Enter => return Some(PbsAction::ViewItem(1)),
                _ => {}
            },
            _ => {}
        }
        None
    }
}

impl Widget for PageSearch {
    fn display(&self, buf: &mut impl BufferAccessor) {
        buf.add(Title("SEARCH".to_string()));

        let h = buf.height() - 1; // removed title
        for (i, item) in self.items.iter().enumerate() {
            let y = i + 1;
            if y >= h {
                break;
            }
            let c = if self.sel == i { '►' } else { ' ' };
            let line = format!(" {} {}", c, item.name());
            buf.put_str(&line, 1, y, Color::Black, Color::White);
        }
    }
}
