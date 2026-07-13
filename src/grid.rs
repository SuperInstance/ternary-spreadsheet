use crate::cell::{Cell, TernaryValue};

/// A 2D grid of ternary cells.
#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    /// Create a new grid filled with neutral cells.
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = (0..rows)
            .map(|_| (0..cols).map(|_| Cell::neutral()).collect())
            .collect();
        Grid { cells, rows, cols }
    }

    /// Grid dimensions as (rows, cols).
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Number of columns.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get a cell reference.
    pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row).and_then(|r| r.get(col))
    }

    /// Get a mutable cell reference.
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(row).and_then(|r| r.get_mut(col))
    }

    /// Set a cell value directly.
    pub fn set(&mut self, row: usize, col: usize, value: TernaryValue) {
        if let Some(cell) = self.get_mut(row, col) {
            cell.set_value(value);
        }
    }

    /// Get a whole row.
    pub fn row(&self, row: usize) -> Option<&[Cell]> {
        self.cells.get(row).map(|r| r.as_slice())
    }

    /// Get a whole column.
    pub fn col(&self, col: usize) -> Option<Vec<Cell>> {
        if col >= self.cols {
            return None;
        }
        Some(self.cells.iter().map(|r| r[col].clone()).collect())
    }

    /// Get all cells flattened.
    pub fn cells(&self) -> Vec<&Cell> {
        self.cells.iter().flat_map(|r| r.iter()).collect()
    }

    /// Get all cells in a range (inclusive).
    pub fn range(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> Vec<&Cell> {
        let (r_min, r_max) = (r1.min(r2), r1.max(r2));
        let (c_min, c_max) = (c1.min(c2), c1.max(c2));
        self.cells
            .iter()
            .enumerate()
            .filter(|(r, _)| *r >= r_min && *r <= r_max)
            .flat_map(|(_, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(c, _)| *c >= c_min && *c <= c_max)
                    .map(|(_, cell)| cell)
            })
            .collect()
    }

    /// Get mutable references to all cells in a range.
    pub fn range_mut(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) -> Vec<&mut Cell> {
        let (r_min, r_max) = (r1.min(r2), r1.max(r2));
        let (c_min, c_max) = (c1.min(c2), c1.max(c2));
        self.cells
            .iter_mut()
            .enumerate()
            .filter(move |(r, _)| *r >= r_min && *r <= r_max)
            .flat_map(move |(_, row)| {
                row.iter_mut()
                    .enumerate()
                    .filter(move |(c, _)| *c >= c_min && *c <= c_max)
                    .map(|(_, cell)| cell)
            })
            .collect()
    }

    /// Apply a function to every cell.
    pub fn for_each<F: FnMut(&mut Cell)>(&mut self, mut f: F) {
        for row in &mut self.cells {
            for cell in row.iter_mut() {
                f(cell);
            }
        }
    }

    /// Total cell count.
    pub fn len(&self) -> usize {
        self.rows * self.cols
    }

    /// Is the grid empty?
    pub fn is_empty(&self) -> bool {
        self.rows == 0 || self.cols == 0
    }

    /// Compute default fitness for all cells.
    pub fn compute_all_fitness(&mut self) {
        for row in &mut self.cells {
            for cell in row.iter_mut() {
                cell.set_fitness(cell.compute_default_fitness());
            }
        }
    }

    /// Get mutable access to all rows (for sorting/reordering).
    pub fn rows_mut(&mut self) -> &mut Vec<Vec<Cell>> {
        &mut self.cells
    }

    /// Parse a cell reference like "A1" into (row, col).
    pub fn parse_cell_ref(s: &str) -> Option<(usize, usize)> {
        let s = s.trim();
        let col_end = s.chars().position(|c| c.is_ascii_digit())?;
        let col_str = &s[..col_end];
        let row_str = &s[col_end..];

        let mut col: usize = 0;
        for ch in col_str.chars() {
            if !ch.is_ascii_alphabetic() {
                return None;
            }
            col = col * 26 + (ch.to_ascii_uppercase() as usize - 'A' as usize + 1);
        }
        if col == 0 {
            return None;
        }
        col -= 1;

        let row: usize = row_str.parse().ok()?;
        if row == 0 {
            return None;
        }
        Some((row - 1, col))
    }
}
