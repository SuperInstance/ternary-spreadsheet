//! Ternary logic spreadsheet engine

use std::collections::HashMap;

/// Cell value supporting ternary logic
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Tri(i8),       // -1, 0, 1
    Number(f64),
    Text(String),
    Empty,
}

/// A spreadsheet cell
#[derive(Debug, Clone)]
pub struct Cell {
    pub value: CellValue,
    pub formula: Option<String>,
}

/// The spreadsheet engine
pub struct Spreadsheet {
    cells: HashMap<(usize, usize), Cell>,
    rows: usize,
    cols: usize,
}

impl Spreadsheet {
    pub fn new(rows: usize, cols: usize) -> Self {
        Spreadsheet { cells: HashMap::new(), rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> &CellValue {
        self.cells.get(&(row, col)).map(|c| &c.value).unwrap_or(&CellValue::Empty)
    }

    pub fn set(&mut self, row: usize, col: usize, value: CellValue) {
        self.cells.insert((row, col), Cell { value, formula: None });
    }

    pub fn set_formula(&mut self, row: usize, col: usize, formula: String) {
        self.cells.insert((row, col), Cell { value: CellValue::Empty, formula: Some(formula) });
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Evaluate ternary AND across a range of cells
    pub fn ternary_and(&self, cells: &[(usize, usize)]) -> i8 {
        cells.iter()
            .filter_map(|&(r, c)| if let CellValue::Tri(v) = self.get(r, c) { Some(*v) } else { None })
            .fold(1i8, |a, b| a.min(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_spreadsheet() {
        let mut ss = Spreadsheet::new(10, 10);
        ss.set(0, 0, CellValue::Tri(1));
        assert_eq!(ss.get(0, 0), &CellValue::Tri(1));
    }
}
