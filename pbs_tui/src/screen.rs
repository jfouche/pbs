use crossterm::{cursor, queue, style};
use std::io;

pub struct Screen {
    width: usize,
    height: usize,
    current: Vec<char>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let mut current = Vec::with_capacity(width * height);
        current.resize(width * height, ' ');
        Screen {
            width,
            height,
            current,
        }
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
        if let Some(v) = self.current.get_mut(idx) {
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

    pub fn render(&self, w: &mut impl io::Write) -> io::Result<()> {
        queue!(w, cursor::MoveTo(0, 0), style::Print('c'))?;
        for (i, c) in self.current.iter().enumerate() {
            let (x, y) = self.coord(i);
            queue!(w, cursor::MoveTo(x as u16, y as u16), style::Print(c))?;
        }
        // self.current.iter().for_each(|c| {
        //     // queue!(w, style::Print(c));
        //     queue!(w, style::Print('*'));
        // });
        // // w.queue(style::Print(self.current))?;
        // for c in self.current.iter() {
        //     queue!(w, style::Print(c))?;
        // }
        w.flush()?;
        Ok(())
    }
}

pub trait Widget {
    fn display(&self, screen: &mut Screen);
}

impl Screen {
    pub fn add(&mut self, w: impl Widget) {
        w.display(self);
    }
}

/// Display a title centered at the top of the screen.
pub struct Title(pub String);

impl Widget for Title {
    fn display(&self, screen: &mut Screen) {
        screen.put_str(&self.0, screen.width() / 2 - self.0.len() / 2, 0);
    }
}

pub struct Paragraph(pub String);

impl Widget for Paragraph {
    fn display(&self, screen: &mut Screen) {
        for (i, line) in self.0.lines().enumerate() {
            screen.put_str(line, 0, i + 3);
        }
    }
}
