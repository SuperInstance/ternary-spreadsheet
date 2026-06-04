use crate::grid::Grid;

/// Axis to sort by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortAxis {
    Row,
    Column,
}

/// Sort rows or columns by their total fitness (descending = fittest first).
pub fn sort_by_fitness(grid: &mut Grid, by: SortAxis) {
    match by {
        SortAxis::Row => {
            let mut rows = std::mem::take(grid.rows_mut());
            rows.sort_by(|a, b| {
                let fa: f64 = a.iter().map(|c| c.fitness).sum();
                let fb: f64 = b.iter().map(|c| c.fitness).sum();
                fb.partial_cmp(&fa).unwrap_or(std::cmp::Ordering::Equal)
            });
            *grid.rows_mut() = rows;
        }
        SortAxis::Column => {
            let (nrows, ncols) = grid.dimensions();
            let mut col_fitness: Vec<(usize, f64)> = (0..ncols)
                .map(|c| {
                    let fitness: f64 = (0..nrows)
                        .filter_map(|r| grid.get(r, c).map(|cell| cell.fitness))
                        .sum();
                    (c, fitness)
                })
                .collect();
            col_fitness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            let col_order: Vec<usize> = col_fitness.iter().map(|(c, _)| *c).collect();
            let mut new_cells = Vec::with_capacity(nrows);
            for r in 0..nrows {
                let mut row = Vec::with_capacity(ncols);
                for &c in &col_order {
                    row.push(grid.get(r, c).cloned().unwrap_or_default());
                }
                new_cells.push(row);
            }
            *grid.rows_mut() = new_cells;
        }
    }
}
