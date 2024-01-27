#[derive(Debug)]
pub struct Cell {
    pub col: u16,
    pub row: u16,
}

impl Cell {
    pub fn new(col: u16, row: u16) -> Self {
        Self { col, row }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(1, 1)
    }
}
