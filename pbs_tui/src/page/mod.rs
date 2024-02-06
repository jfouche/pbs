use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::widget::Buffer;
use crate::widget::Widget;
use crate::PbsAction;

use self::{help::PageHelp, search::PageSearch};

mod help;
mod search;

pub enum Page {
    Help(PageHelp),
    Search(PageSearch),
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
        }
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Action> {
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
                // CTRL-s : Shortcut to PageSeach
                *self = Page::Search(PageSearch::default());
                return None;
            } else if key.code == KeyCode::Char('h')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                // CTRL-h : Shortcut to PageHelp
                *self = Page::Help(PageHelp);
                return None;
            }
        }

        match self {
            Page::Help(page) => {
                page.handle_event(event);
                None
            }
            Page::Search(page) => page.handle_event(event),
        }
    }
}
