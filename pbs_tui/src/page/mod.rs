use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::PbsAction;
use crate::{buffer::BufferAccessor, widget::Widget};

use self::{help::PageHelp, new_item::PageMakeItem, search::PageSearch};

mod help;
mod new_item;
mod search;

pub enum Page {
    Help(PageHelp),
    Search(PageSearch),
    MakeItem(PageMakeItem),
}

impl Page {
    pub fn home() -> Self {
        Page::Help(PageHelp {})
    }
}

impl Page {
    pub fn handle_event(&mut self, event: &Event) -> Option<PbsAction> {
        if let Event::Key(key) = event {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('s') => {
                        // CTRL-s : Shortcut to PageSeach
                        *self = Page::Search(PageSearch::default());
                        return None;
                    }
                    KeyCode::Char('h') => {
                        // CTRL-h : Shortcut to PageHelp
                        *self = Page::Help(PageHelp);
                        return None;
                    }
                    KeyCode::Char('n') => {
                        // CTRL-n : Shortcut to PageMakeItem
                        *self = Page::MakeItem(PageMakeItem);
                        return None;
                    }
                    _ => {}
                }
            }
        }

        match self {
            Page::Help(_) => None,
            Page::Search(page) => page.handle_event(event),
            Page::MakeItem(_) => None,
        }
    }
}

impl Widget for Page {
    fn display(&self, buf: &mut impl BufferAccessor) {
        match self {
            Page::Help(page) => page.display(buf),
            Page::Search(page) => page.display(buf),
            Page::MakeItem(page) => page.display(buf),
        }
    }
}
