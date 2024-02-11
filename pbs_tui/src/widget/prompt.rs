use crossterm::{
    event::{Event, KeyCode, KeyEventKind, KeyModifiers},
    style::Color,
};

use super::{Buffer, Widget};

#[derive(Default)]
pub struct Prompt {
    label: String,
    input: String,
}

pub enum PromptEvent {
    Updated(String),
    Entered(String),
}

impl Prompt {
    pub fn set_label(&mut self, label: impl ToString) {
        self.label = label.to_string();
    }
}

impl Widget for Prompt {
    type Action = PromptEvent;

    fn display(&self, buf: &mut Buffer) {
        let s = format!("{}{}", self.label, self.input);
        buf.put_str(&s, 0, buf.height() - 1, Color::Black, Color::White);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Action> {
        match event {
            Event::Key(key_evt)
                if !key_evt.modifiers.contains(KeyModifiers::CONTROL)
                    && !key_evt.modifiers.contains(KeyModifiers::ALT)
                    && key_evt.kind == KeyEventKind::Press =>
            {
                match key_evt.code {
                    KeyCode::Char(c) => {
                        self.input.push(c);
                        return Some(PromptEvent::Updated(self.input.clone()));
                    }
                    KeyCode::Backspace => {
                        self.input.pop();
                        return Some(PromptEvent::Updated(self.input.clone()));
                    }
                    KeyCode::Enter => {
                        let input = self.input.clone();
                        self.input.clear();
                        return Some(PromptEvent::Entered(input));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        None
    }
}
