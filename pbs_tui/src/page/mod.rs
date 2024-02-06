use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::widget::Buffer;
use crate::widget::Widget;

use self::{help::PageHelp, search::PageSeach};

mod help;
mod search;

pub enum Page {
    Help(PageHelp),
    Search(PageSeach),
}

impl Page {
    pub fn home() -> Self {
        Page::Help(PageHelp {})
    }

    pub fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
                *self = Page::Search(PageSeach::new());
                return;
            } else if key.code == KeyCode::Char('h')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                *self = Page::Help(PageHelp::new());
                return;
            }
        }

        match self {
            Page::Help(page) => page.handle_event(event),
            Page::Search(page) => page.handle_event(event),
        }
    }
}

impl Widget for &Page {
    fn display(&self, buf: &mut Buffer) {
        match self {
            Page::Help(page) => page.display(buf),
            Page::Search(page) => page.display(buf),
        }
    }
}
