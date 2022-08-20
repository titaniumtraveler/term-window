use std::fmt::{self, Write};

use crate::window::Shape;

pub struct Pane<'a, W: Write> {
    shape: &'a Shape,
    writer: W,
}

impl<'a, W: Write> Pane<'a, W> {
    pub fn new(shape: &'a Shape, writer: W) -> Self {
        Self { shape, writer }
    }

    pub fn shape(&self) -> &'a Shape {
        self.shape
    }
}

impl<W: Write> Write for Pane<'_, W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)
    }
}
