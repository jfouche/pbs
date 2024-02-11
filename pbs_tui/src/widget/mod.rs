mod paragraph;
mod prompt;
mod statusbar;
mod title;

use crossterm::{event::Event, style::Color};
pub use paragraph::Paragraph;
pub use prompt::{Prompt, PromptEvent};
pub use statusbar::StatusBar;
pub use title::Title;

pub trait Widget {
    type Action;

    fn display(&self, buf: &mut Buffer);

    fn handle_event(&mut self, _event: &Event) -> Option<Self::Action> {
        None
    }
}

impl<T: Widget> Widget for &mut T {
    type Action = T::Action;
    fn display(&self, buf: &mut Buffer) {
        (**self).display(buf);
    }

    fn handle_event(&mut self, event: &Event) -> Option<Self::Action> {
        (**self).handle_event(event)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub c: char,
    pub bg_color: Color,
    pub fg_color: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            c: ' ',
            bg_color: Color::Black,
            fg_color: Color::White,
        }
    }
}

#[derive(Clone)]
pub struct Buffer {
    width: usize,
    height: usize,
    buf: Vec<Cell>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            buf: vec![Cell::default(); width * height],
        }
    }

    pub fn reset(&mut self) {
        self.reset_with(Cell::default());
    }

    pub fn reset_with(&mut self, cell: Cell) {
        self.buf.fill(cell);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn coord(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    /// Put a char on the screen. The coordinates start at 0
    pub fn put_char(&mut self, c: char, x: usize, y: usize, bg_color: Color, fg_color: Color) {
        assert!(x < self.width && y < self.height);
        let idx = self.idx(x, y);
        if let Some(v) = self.buf.get_mut(idx) {
            *v = Cell {
                c,
                bg_color,
                fg_color,
            }
        };
    }

    /// put a str on the screen. If the str go out of the screen, it will
    /// print `…` to show ellision
    // TODO : &self.current[x..x + s.len()].copy_from_slice(s[..]);
    pub fn put_str(&mut self, s: &str, x: usize, y: usize, bg_color: Color, fg_color: Color) {
        assert!(x < self.width && y < self.height);
        if x + s.len() <= self.width {
            // There is enough space in line
            for (i, c) in s.chars().enumerate() {
                self.put_char(c, x + i, y, bg_color, fg_color);
            }
        } else {
            // The string is too long, limit it and append […]
            let offset = self.width - x - 1; // -1 for ...
            for (i, c) in s[0..offset].chars().enumerate() {
                self.put_char(c, x + i, y, bg_color, fg_color);
            }
            self.put_char('…', x + offset, y, bg_color, fg_color);
        }
    }

    pub fn add(&mut self, w: impl Widget) {
        w.display(self);
    }

    /// create the diffs betwen 2 buffers
    pub fn diff(&self, other: &Self) -> Vec<Patch> {
        assert!(self.width == other.width && self.height == other.height);
        self.buf
            .iter()
            .zip(other.buf.iter())
            .enumerate()
            .filter(|(_i, (curr, prev))| curr != prev)
            .map(|(i, (curr, _prev))| {
                let (x, y) = self.coord(i);
                Patch { cell: *curr, x, y }
            })
            .collect()
    }
}
pub struct Patch {
    pub x: usize,
    pub y: usize,
    pub cell: Cell,
}
