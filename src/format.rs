use crate::cell::Cell;

/// Color assigned based on fitness score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitnessColor {
    /// Red — avoid (negative fitness)
    Red,
    /// Yellow — unknown/neutral (near zero)
    Yellow,
    /// Green — choose (positive fitness)
    Green,
}

impl std::fmt::Display for FitnessColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FitnessColor::Red => write!(f, "red"),
            FitnessColor::Yellow => write!(f, "yellow"),
            FitnessColor::Green => write!(f, "green"),
        }
    }
}

/// Assign a fitness color to a cell.
pub fn conditional_format(cell: &Cell) -> FitnessColor {
    if cell.fitness > 0.5 {
        FitnessColor::Green
    } else if cell.fitness < -0.5 {
        FitnessColor::Red
    } else {
        FitnessColor::Yellow
    }
}

/// Assign fitness colors with custom thresholds.
pub fn conditional_format_with_thresholds(
    cell: &Cell,
    green_threshold: f64,
    red_threshold: f64,
) -> FitnessColor {
    if cell.fitness >= green_threshold {
        FitnessColor::Green
    } else if cell.fitness <= red_threshold {
        FitnessColor::Red
    } else {
        FitnessColor::Yellow
    }
}

/// Format a slice of cells into their color codes.
pub fn format_cells(cells: &[Cell]) -> Vec<FitnessColor> {
    cells.iter().map(conditional_format).collect()
}
