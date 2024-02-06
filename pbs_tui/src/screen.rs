use crossterm::{cursor, style, QueueableCommand};
use std::io;

use crate::widget::{Buffer, Widget};

pub struct Screen {
    previous_buffer: Buffer,
    current_buffer: Buffer,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Screen {
            previous_buffer: Buffer::new(width, height),
            current_buffer: Buffer::new(width, height),
        }
    }

    pub fn render(&mut self, w: &mut impl io::Write) -> io::Result<()> {
        let diff = self.current_buffer.diff(&self.previous_buffer);
        for cell in diff.iter() {
            w.queue(cursor::MoveTo(cell.x as u16, cell.y as u16))?
                .queue(style::Print(cell.c))?;
        }
        self.previous_buffer = self.current_buffer.clone();
        self.current_buffer.reset();
        w.flush()?;
        Ok(())
    }

    pub fn add(&mut self, w: impl Widget) {
        self.current_buffer.add(w);
    }
}
