use super::cell::Cell;

/// Represents a line buffer in the Read-Eval-Print Loop (REPL).
#[derive(Debug)]
pub struct LineBuffer {
    /// The string buffer containing the input line.
    pub buffer: String,
    /// The caret position within the line buffer, represented by a cell in the terminal.
    pub caret: Cell,
}

impl LineBuffer {
    /// Creates a new `LineBuffer` with default values.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `LineBuffer` instance.
    pub fn new() -> Self {
        LineBuffer::default()
    }
}

impl Default for LineBuffer {
    /// Creates a default `LineBuffer` with an empty buffer and a caret at the default position.
    ///
    /// # Returns
    ///
    /// * `Self` - A default `LineBuffer` instance.
    fn default() -> Self {
        Self {
            buffer: String::new(),
            caret: Cell::default(),
        }
    }
}
