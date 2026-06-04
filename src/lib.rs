//! # Ternary Spreadsheet
//!
//! Core logic for the SuperInstance Spreadsheet — a familiar spreadsheet interface
//! where every cell is a tiny ternary intelligence.

mod cell;
mod grid;
mod formula;
mod sort;
mod autofill;
mod format;
mod heatmap;

pub use cell::{TernaryValue, Cell};
pub use grid::Grid;
pub use formula::{FormulaEngine, FormulaError, FormulaResult};
pub use sort::{sort_by_fitness, SortAxis};
pub use autofill::{autofill_mutate, MutationConfig};
pub use format::{conditional_format, conditional_format_with_thresholds, format_cells, FitnessColor};
pub use heatmap::fitness_heatmap;

#[cfg(test)]
mod tests;
