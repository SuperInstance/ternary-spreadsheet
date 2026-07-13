//! Ternary logic spreadsheet engine.
//!
//! Every cell holds a value in `{-1, 0, +1}`. Supports formula evaluation
//! (`SUM`, `AVG`, `COUNT`, `ENTROPY`, `EVOLVE`, `BEST`, `SPECIES`,
//! `EXHAUSTIVE`), conditional formatting via fitness coloring, mutation-based
//! autofill, and fitness-sorted row/column operations.

mod autofill;
mod cell;
mod format;
mod formula;
mod grid;
mod heatmap;
mod sort;

pub use autofill::{autofill_mutate, MutationConfig};
pub use cell::{Cell, TernaryValue};
pub use format::{
    conditional_format, conditional_format_with_thresholds, format_cells, FitnessColor,
};
pub use formula::{FormulaEngine, FormulaError, FormulaResult};
pub use grid::Grid;
pub use heatmap::{fitness_heatmap, fitness_heatmap_range};
pub use sort::{sort_by_fitness, SortAxis};

#[cfg(test)]
mod tests;
