use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::widget::Buffer;
use crate::widget::Widget;
use crate::PbsAction;

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

impl Widget for Page {
    type Action = PbsAction;

    fn display(&self, buf: &mut Buffer) {
        match self {
            Page::Help(page) => page.display(buf),
            Page::Search(page) => page.display(buf),
            Page::MakeItem(page) => page.display(buf),
        }
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Action> {
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
            Page::Help(page) => {
                page.handle_event(event);
                None
            }
            Page::Search(page) => page.handle_event(event),
            Page::MakeItem(page) => {
                page.handle_event(event);
                None
            }
        }
    }
}
