use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::screen::Screen;

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

    pub fn display(&mut self, w: &mut Screen) {
        match self {
            Page::Help(page) => page.display(w),
            Page::Search(page) => page.display(w),
        }
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
