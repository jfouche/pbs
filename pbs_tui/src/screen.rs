use crossterm::{
    cursor,
    style::{self, Color, SetBackgroundColor},
    QueueableCommand,
};
use std::io;

use crate::widget::{Buffer, Cell, Widget};

pub struct Screen {
    previous_buffer: Buffer,
    current_buffer: Buffer,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let current_buffer = Buffer::new(width, height);
        let mut previous_buffer = Buffer::new(width, height);
        // Fill the previous buffer with chars, to force full redraw on first render
        previous_buffer.reset_with(Cell {
            c: 'x',
            bg_color: Color::Yellow,
            fg_color: Color::Yellow,
        });

        Screen {
            previous_buffer,
            current_buffer,
        }
    }

    pub fn render(&mut self, w: &mut impl io::Write) -> io::Result<()> {
        let diff = self.current_buffer.diff(&self.previous_buffer);
        let mut curr_bg_color = Color::Reset;
        let mut curr_fg_color = Color::Reset;
        for patch in diff.iter() {
            let cell = patch.cell;
            if cell.bg_color != curr_bg_color {
                w.queue(SetBackgroundColor(cell.bg_color))?;
                curr_bg_color = cell.bg_color;
            }
            if cell.fg_color != curr_fg_color {
                w.queue(SetBackgroundColor(cell.bg_color))?;
                curr_fg_color = cell.fg_color;
            }
            w.queue(cursor::MoveTo(patch.x as u16, patch.y as u16))?
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
