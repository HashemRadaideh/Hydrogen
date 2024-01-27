use super::cell::Cell;

#[derive(Debug)]
pub struct LineBuffer {
    pub buffer: String,
    pub caret: Cell,
}

impl LineBuffer {
    pub fn new() -> Self {
        LineBuffer::default()
    }
}

impl Default for LineBuffer {
    fn default() -> Self {
        Self {
            buffer: String::new(),
            caret: Cell::default(),
        }
    }
}
