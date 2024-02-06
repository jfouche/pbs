use crossterm::event::{Event, KeyCode, KeyEventKind};

use super::{Buffer, Widget};

#[derive(Default)]
pub struct Prompt {
    label: String,
    input: String,
}

impl Prompt {
    pub fn set_label(&mut self, label: impl ToString) {
        self.label = label.to_string();
    }
}

impl Widget for Prompt {
    fn display(&self, buf: &mut Buffer) {
        let s = format!("{}{}", self.label, self.input);
        buf.put_str(&s, 0, buf.height() - 1);
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Key(key_evt)
                if key_evt.modifiers.is_empty() && key_evt.kind == KeyEventKind::Press =>
            {
                match key_evt.code {
                    KeyCode::Char(c) => {
                        self.input.push(c);
                    }
                    KeyCode::Backspace => {
                        self.input.pop();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
