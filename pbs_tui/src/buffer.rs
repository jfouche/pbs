use crossterm::style::Color;

use crate::widget::Widget;

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
    cursor: (usize, usize),
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            buf: vec![Cell::default(); width * height],
            cursor: (0, 0),
        }
    }

    pub fn resize_with(&mut self, w: usize, h: usize, cell: Cell) {
        self.width = w;
        self.height = h;
        self.buf.resize(w * h, cell);
        self.buf.fill(cell);
        // TODO : move cursor if necessary
    }

    pub fn reset(&mut self) {
        self.reset_with(Cell::default());
    }

    pub fn reset_with(&mut self, cell: Cell) {
        self.buf.fill(cell);
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn coord(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
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

pub struct Bound {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Bound {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Bound { x, y, w, h }
    }
}

pub struct BufferView<'a> {
    buf: &'a mut Buffer,
    bound: Bound,
}

impl<'a> BufferView<'a> {
    pub fn from_buffer(buf: &'a mut Buffer, bound: Bound) -> Self {
        BufferView { buf, bound }
    }
    pub fn from_view(buf: &'a mut BufferView, bound: Bound) -> Self {
        BufferView {
            buf: buf.buf,
            bound,
        }
    }
}

pub trait BufferAccessor {
    fn view(&mut self, bound: Bound) -> BufferView;

    /// Put a char on the screen. The coordinates start at 0
    fn put_char(&mut self, c: char, x: usize, y: usize, bg_color: Color, fg_color: Color);

    /// put a str on the screen. If the str go out of the screen, it will
    /// print `…` to show ellision
    /// Returns the x position at the end of the str
    fn put_str(&mut self, s: &str, x: usize, y: usize, bg_color: Color, fg_color: Color) -> usize;

    /// Width of the buffer
    fn width(&self) -> usize;

    /// Height of the buffer
    fn height(&self) -> usize;

    // Add a widget
    fn add(&mut self, w: impl Widget);

    /// Position the cursor
    fn set_cursor(&mut self, x: usize, y: usize);
}

impl BufferAccessor for Buffer {
    fn view(&mut self, bound: Bound) -> BufferView {
        BufferView::from_buffer(self, bound)
    }

    fn put_char(&mut self, c: char, x: usize, y: usize, bg_color: Color, fg_color: Color) {
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

    // TODO : &self.current[x..x + s.len()].copy_from_slice(s[..]);
    fn put_str(&mut self, s: &str, x: usize, y: usize, bg_color: Color, fg_color: Color) -> usize {
        assert!(
            x < self.width && y < self.height,
            "{x} < {w}, {y} < {h} : {s}",
            w = self.width,
            h = self.height
        );
        if x + s.len() <= self.width {
            // There is enough space in line
            for (i, c) in s.chars().enumerate() {
                self.put_char(c, x + i, y, bg_color, fg_color);
            }
            x + s.len()
        } else {
            // The string is too long, limit it and append […]
            let offset = self.width - x - 1; // -1 for ...
            for (i, c) in s[0..offset].chars().enumerate() {
                self.put_char(c, x + i, y, bg_color, fg_color);
            }
            self.put_char('…', x + offset, y, bg_color, fg_color);
            self.width
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn add(&mut self, w: impl Widget) {
        w.display(self);
    }

    fn set_cursor(&mut self, x: usize, y: usize) {
        self.cursor = (x, y);
    }
}

impl<'a> BufferAccessor for BufferView<'a> {
    fn view(&mut self, bound: Bound) -> BufferView {
        BufferView::from_view(self, bound)
    }

    fn put_char(&mut self, c: char, x: usize, y: usize, bg_color: Color, fg_color: Color) {
        let x = self.bound.x + x;
        let y = self.bound.y + y;
        if x < self.bound.w {
            self.buf.put_char(c, x, y, bg_color, fg_color)
        }
    }

    fn put_str(&mut self, s: &str, x: usize, y: usize, bg_color: Color, fg_color: Color) -> usize {
        let x = self.bound.x + x;
        let y = self.bound.y + y;
        self.buf.put_str(s, x, y, bg_color, fg_color)
    }

    fn add(&mut self, w: impl Widget) {
        self.buf.add(w)
    }

    fn width(&self) -> usize {
        self.bound.w
    }

    fn height(&self) -> usize {
        self.bound.h
    }

    fn set_cursor(&mut self, x: usize, y: usize) {
        self.buf.set_cursor(self.bound.x + x, self.bound.y + y)
    }
}
