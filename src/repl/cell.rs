/// Represents a cell in the terminal, defined by its column and row positions.
#[derive(Debug)]
pub struct Cell {
    /// The column position of the cell.
    pub col: u16,
    /// The row position of the cell.
    pub row: u16,
}

impl Cell {
    /// Creates a new `Cell` with the specified column and row positions.
    ///
    /// # Arguments
    ///
    /// * `col` - The column position of the cell.
    /// * `row` - The row position of the cell.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `Cell` instance.
    pub fn new(col: u16, row: u16) -> Self {
        Self { col, row }
    }
}

impl Default for Cell {
    /// Creates a default `Cell` with column and row positions set to 1.
    ///
    /// # Returns
    ///
    /// * `Self` - A default `Cell` instance.
    fn default() -> Self {
        Cell::new(1, 1)
    }
}
