use crate::grid::Grid;

/// Compute a heatmap of fitness values for the entire grid.
/// Returns a Vec<Vec<f64>> where each value is normalized to [0.0, 1.0].
pub fn fitness_heatmap(grid: &Grid) -> Vec<Vec<f64>> {
    let (rows, cols) = grid.dimensions();

    // Find min and max fitness
    let mut min_fit = f64::INFINITY;
    let mut max_fit = f64::NEG_INFINITY;

    for r in 0..rows {
        for c in 0..cols {
            if let Some(cell) = grid.get(r, c) {
                min_fit = min_fit.min(cell.fitness);
                max_fit = max_fit.max(cell.fitness);
            }
        }
    }

    let range = max_fit - min_fit;

    let mut heatmap = Vec::with_capacity(rows);
    for r in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for c in 0..cols {
            let normalized = if let Some(cell) = grid.get(r, c) {
                if range.abs() < f64::EPSILON {
                    0.5 // All same fitness -> middle of heatmap
                } else {
                    (cell.fitness - min_fit) / range
                }
            } else {
                0.0
            };
            row.push(normalized);
        }
        heatmap.push(row);
    }

    heatmap
}

/// Compute heatmap for a specific range.
pub fn fitness_heatmap_range(
    grid: &Grid,
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
) -> Vec<Vec<f64>> {
    let (r_min, r_max) = (r1.min(r2), r1.max(r2));
    let (c_min, c_max) = (c1.min(c2), c1.max(c2));

    let mut min_fit = f64::INFINITY;
    let mut max_fit = f64::NEG_INFINITY;

    for r in r_min..=r_max {
        for c in c_min..=c_max {
            if let Some(cell) = grid.get(r, c) {
                min_fit = min_fit.min(cell.fitness);
                max_fit = max_fit.max(cell.fitness);
            }
        }
    }

    let range = max_fit - min_fit;

    let mut heatmap = Vec::new();
    for r in r_min..=r_max {
        let mut row = Vec::new();
        for c in c_min..=c_max {
            let normalized = if let Some(cell) = grid.get(r, c) {
                if range.abs() < f64::EPSILON {
                    0.5
                } else {
                    (cell.fitness - min_fit) / range
                }
            } else {
                0.0
            };
            row.push(normalized);
        }
        heatmap.push(row);
    }

    heatmap
}
