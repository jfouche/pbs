use crossterm::event::Event;

use crate::{
    page::Page,
    widget::{self, Prompt, PromptEvent, StatusBar, Widget},
    PbsAction, PbsResponse,
};

pub struct MainWindow {
    page: Page,
    status: StatusBar,
    prompt: Prompt,
}

impl MainWindow {
    pub fn new() -> Self {
        MainWindow {
            page: Page::home(),
            status: StatusBar::default(),
            prompt: Prompt::default(),
        }
    }

    pub fn handle_response(&mut self, response: PbsResponse) {
        match (response, &mut self.page) {
            (PbsResponse::Err(err), _) => {
                self.status.text = err;
            }
            (PbsResponse::Items(items), Page::Search(ref mut page)) => {
                page.set_items(items);
            }
            (PbsResponse::Item(item), Page::MakeItem(ref mut _page)) => {
                self.status.text = format!("Item {} created", item.pn());
            }
            _ => {}
        }
    }
}

impl Widget for MainWindow {
    type Action = PbsAction;

    fn display(&self, buf: &mut widget::Buffer) {
        self.page.display(buf);
        self.status.display(buf);
        self.prompt.display(buf);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Action> {
        match self.page {
            Page::Help(_) => self.prompt.set_label("> "),
            Page::Search(_) => self.prompt.set_label("search> "),
            Page::MakeItem(_) => self.prompt.set_label("item name> "),
        }

        self.page.handle_event(event);
        if let Some(prompt_evt) = self.prompt.handle_event(event) {
            match (&self.page, prompt_evt) {
                (Page::Search(_), PromptEvent::Updated(text)) if text.len() > 2 => {
                    return Some(PbsAction::Search(text));
                }
                (Page::MakeItem(_), PromptEvent::Entered(text)) => {
                    return Some(PbsAction::CreateItem(text));
                }
                _ => {}
            }
        }
        None
    }
}
