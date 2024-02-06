mod paragraph;
mod title;

pub use paragraph::Paragraph;
pub use title::Title;

pub trait Widget {
    fn display(&self, buf: &mut Buffer);
}

#[derive(Clone)]
pub struct Buffer {
    width: usize,
    height: usize,
    buf: Vec<char>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            buf: vec![' '; width * height],
        }
    }

    pub fn reset(&mut self) {
        self.buf.fill(' ');
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn coord(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    /// Put a char on the screen. The coordinates start at 0
    pub fn put_char(&mut self, c: char, x: usize, y: usize) {
        assert!(x < self.width && y < self.height);
        let idx = self.idx(x, y);
        if let Some(v) = self.buf.get_mut(idx) {
            *v = c
        };
    }

    /// put a str on the screen. If the str go out of the screen, it will
    /// print `...` to show ellision
    pub fn put_str(&mut self, s: &str, x: usize, y: usize) {
        assert!(x < self.width && y < self.height);
        if x + s.len() < self.width {
            // TODO : &self.current[x..x + s.len()].copy_from_slice(s[..]);
            for (i, c) in s.chars().enumerate() {
                self.put_char(c, x + i, y);
            }
        }
    }

    pub fn add(&mut self, w: impl Widget) {
        w.display(self);
    }

    /// create the diffs betwen 2 buffers
    pub fn diff(&self, other: &Self) -> Vec<Cell> {
        assert!(self.width == other.width && self.height == other.height);
        self.buf
            .iter()
            .zip(other.buf.iter())
            .enumerate()
            .filter(|(_i, (cc, pc))| cc != pc)
            .map(|(i, (cc, _pc))| {
                let (x, y) = self.coord(i);
                Cell { c: *cc, x, y }
            })
            .collect()
    }
}

pub struct Cell {
    pub c: char,
    pub x: usize,
    pub y: usize,
}
