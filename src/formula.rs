use crate::cell::{Cell, TernaryValue};
use crate::grid::Grid;

/// Result of a formula evaluation.
pub type FormulaResult = Result<f64, FormulaError>;

/// Errors that can occur during formula evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum FormulaError {
    UnknownFunction(String),
    InvalidArguments(String),
    InvalidRange(String),
    DivisionByZero,
    NoData,
}

impl std::fmt::Display for FormulaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaError::UnknownFunction(name) => write!(f, "unknown function: {}", name),
            FormulaError::InvalidArguments(msg) => write!(f, "invalid arguments: {}", msg),
            FormulaError::InvalidRange(msg) => write!(f, "invalid range: {}", msg),
            FormulaError::DivisionByZero => write!(f, "division by zero"),
            FormulaError::NoData => write!(f, "no data"),
        }
    }
}

/// The formula engine evaluates spreadsheet formulas against a grid.
pub struct FormulaEngine {
    grid: Grid,
}

impl FormulaEngine {
    /// Create a new engine wrapping a grid.
    pub fn new(grid: Grid) -> Self {
        FormulaEngine { grid }
    }

    /// Get a reference to the underlying grid.
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Get a mutable reference to the underlying grid.
    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    /// Evaluate a formula string like `=EVOLVE(A1:A10, 100)`.
    pub fn evaluate(&mut self, formula: &str) -> FormulaResult {
        let formula = formula.trim();
        let formula = formula.strip_prefix('=').unwrap_or(formula);

        // Parse function name and arguments
        let paren_pos = formula
            .find('(')
            .ok_or_else(|| FormulaError::InvalidArguments("expected function(args)".into()))?;
        let name = formula[..paren_pos].trim().to_uppercase();
        let inner = formula[paren_pos + 1..].trim();
        let inner = inner
            .strip_suffix(')')
            .ok_or_else(|| FormulaError::InvalidArguments("missing closing parenthesis".into()))?
            .trim();

        match name.as_str() {
            "EVOLVE" => self.evolve(inner),
            "BEST" => self.best(inner),
            "SPECIES" => self.species(inner),
            "EXHAUSTIVE" => self.exhaustive(inner),
            "ENTROPY" => self.entropy(inner),
            "SUM" => self.sum(inner),
            "AVG" => self.avg(inner),
            "COUNT" => self.count(inner),
            _ => Err(FormulaError::UnknownFunction(name)),
        }
    }

    /// Parse a range like "A1:B5" or "A:A" into cell lists.
    fn parse_range_cells(&self, range_str: &str) -> Result<Vec<Cell>, FormulaError> {
        let range_str = range_str.trim();

        if range_str.contains(':') {
            let parts: Vec<&str> = range_str.splitn(2, ':').collect();
            let start = parts[0].trim();
            let end = parts[1].trim();

            // Full column range like "A:A"
            if start.chars().all(|c| c.is_ascii_alphabetic())
                && end.chars().all(|c| c.is_ascii_alphabetic())
            {
                let col_start = Grid::parse_cell_ref(&format!("{}1", start));
                let col_end = Grid::parse_cell_ref(&format!("{}{}", end, self.grid.rows()));
                match (col_start, col_end) {
                    (Some((_, cs)), Some((_, ce))) if cs == ce => {
                        Ok(self.grid.col(cs).unwrap_or_default())
                    }
                    _ => Err(FormulaError::InvalidRange(range_str.into())),
                }
            } else {
                let start_pos = Grid::parse_cell_ref(start)
                    .ok_or_else(|| FormulaError::InvalidRange(start.into()))?;
                let end_pos = Grid::parse_cell_ref(end)
                    .ok_or_else(|| FormulaError::InvalidRange(end.into()))?;
                Ok(self
                    .grid
                    .range(start_pos.0, start_pos.1, end_pos.0, end_pos.1)
                    .into_iter()
                    .cloned()
                    .collect())
            }
        } else {
            // Single cell
            let pos = Grid::parse_cell_ref(range_str)
                .ok_or_else(|| FormulaError::InvalidRange(range_str.into()))?;
            self.grid
                .get(pos.0, pos.1)
                .cloned()
                .map(|c| vec![c])
                .ok_or_else(|| FormulaError::InvalidRange(range_str.into()))
        }
    }

    /// EVOLVE(range, generations) — run evolution on a range.
    /// Each generation: compute fitness, keep top 50%, mutate rest.
    /// Returns the best fitness achieved.
    fn evolve(&mut self, args: &str) -> FormulaResult {
        let parts: Vec<&str> = args.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(FormulaError::InvalidArguments(
                "EVOLVE(range, generations)".into(),
            ));
        }
        let range_str = parts[0].trim();
        let generations: u32 = parts[1]
            .trim()
            .parse()
            .map_err(|_| FormulaError::InvalidArguments("generations must be a number".into()))?;

        // First get cell positions from range
        let positions = self.get_range_positions(range_str)?;
        if positions.is_empty() {
            return Err(FormulaError::NoData);
        }

        let mut best_fitness = f64::NEG_INFINITY;

        for gen in 0..generations {
            // Compute fitness for cells in range
            for &(r, c) in &positions {
                if let Some(cell) = self.grid.get_mut(r, c) {
                    cell.set_fitness(cell.compute_default_fitness());
                }
            }

            // Sort by fitness, keep top half, mutate bottom half
            let mut with_fitness: Vec<(f64, usize, usize)> = positions
                .iter()
                .filter_map(|&(r, c)| self.grid.get(r, c).map(|cell| (cell.fitness, r, c)))
                .collect();
            with_fitness.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

            let cutoff = with_fitness.len().div_ceil(2);
            for (_, r, c) in with_fitness.into_iter().skip(cutoff) {
                if let Some(cell) = self.grid.get_mut(r, c) {
                    let new_val =
                        TernaryValue::from_seed(gen * 17 + (r as u32) * 31 + (c as u32) * 7 + 13);
                    cell.set_value(new_val);
                }
            }

            // Track best
            if let Some(cell) = self.grid.get(positions[0].0, positions[0].1) {
                best_fitness = best_fitness.max(cell.fitness);
            }
        }

        // Final fitness check
        for &(r, c) in &positions {
            if let Some(cell) = self.grid.get(r, c) {
                best_fitness = best_fitness.max(cell.fitness);
            }
        }

        Ok(best_fitness)
    }

    /// BEST(range) — return the highest fitness in a range.
    fn best(&self, args: &str) -> FormulaResult {
        let cells = self.parse_range_cells(args.trim())?;
        if cells.is_empty() {
            return Err(FormulaError::NoData);
        }
        cells
            .iter()
            .map(|c| c.fitness)
            .fold(None, |acc: Option<f64>, f| {
                Some(acc.map_or(f, |best| best.max(f)))
            })
            .ok_or(FormulaError::NoData)
    }

    /// SPECIES(range) — count unique species (clusters of same-sign values).
    fn species(&self, args: &str) -> FormulaResult {
        let cells = self.parse_range_cells(args.trim())?;
        if cells.is_empty() {
            return Err(FormulaError::NoData);
        }

        let mut species_count = 0usize;
        let mut prev_sign: Option<i8> = None;

        for cell in &cells {
            let sign = cell.value.as_i8().signum();
            match prev_sign {
                Some(p) if p == sign => {} // same species
                _ => {
                    if sign != 0 {
                        species_count += 1;
                    }
                }
            }
            if sign != 0 {
                prev_sign = Some(sign);
            }
        }

        Ok(species_count as f64)
    }

    /// EXHAUSTIVE(range) — try all possible value combinations and return best total fitness.
    /// For performance, limited to ranges of ≤ 10 cells.
    fn exhaustive(&mut self, args: &str) -> FormulaResult {
        let positions = self.get_range_positions(args.trim())?;
        if positions.is_empty() {
            return Err(FormulaError::NoData);
        }
        if positions.len() > 10 {
            return Err(FormulaError::InvalidArguments(
                "EXHAUSTIVE limited to 10 cells (3^10 = 59049 combinations)".into(),
            ));
        }

        let n = positions.len();
        let combos = 3usize.pow(n as u32);
        let mut best_total = f64::NEG_INFINITY;

        // Save original values
        let original: Vec<TernaryValue> = positions
            .iter()
            .map(|&(r, c)| {
                self.grid
                    .get(r, c)
                    .map(|c| c.value)
                    .unwrap_or(TernaryValue::Neutral)
            })
            .collect();

        for combo in 0..combos {
            let mut val = combo;
            for &(r, c) in &positions {
                let tv = TernaryValue::from_seed(val as u32);
                if let Some(cell) = self.grid.get_mut(r, c) {
                    cell.set_value(tv);
                }
                val /= 3;
            }

            // Compute total fitness
            let total: f64 = positions
                .iter()
                .filter_map(|&(r, c)| {
                    let cell = self.grid.get_mut(r, c)?;
                    cell.set_fitness(cell.compute_default_fitness());
                    Some(cell.fitness)
                })
                .sum();

            best_total = best_total.max(total);
        }

        // Restore original values
        for (i, &(r, c)) in positions.iter().enumerate() {
            if let Some(cell) = self.grid.get_mut(r, c) {
                cell.set_value(original[i]);
            }
        }

        Ok(best_total)
    }

    /// ENTROPY(range) — compute Shannon entropy of ternary values.
    fn entropy(&self, args: &str) -> FormulaResult {
        let cells = self.parse_range_cells(args.trim())?;
        if cells.is_empty() {
            return Err(FormulaError::NoData);
        }

        let n = cells.len() as f64;
        let mut counts = [0usize; 3]; // neg, neutral, pos

        for cell in &cells {
            match cell.value {
                TernaryValue::Negative => counts[0] += 1,
                TernaryValue::Neutral => counts[1] += 1,
                TernaryValue::Positive => counts[2] += 1,
            }
        }

        let mut entropy = 0.0;
        for &count in &counts {
            if count > 0 {
                let p = count as f64 / n;
                entropy -= p * p.log2();
            }
        }

        Ok(entropy)
    }

    /// SUM(range) — sum all cell values.
    fn sum(&self, args: &str) -> FormulaResult {
        let cells = self.parse_range_cells(args.trim())?;
        if cells.is_empty() {
            return Err(FormulaError::NoData);
        }
        Ok(cells.iter().map(|c| c.value.as_i8() as f64).sum())
    }

    /// AVG(range) — average of all cell values.
    fn avg(&self, args: &str) -> FormulaResult {
        let cells = self.parse_range_cells(args.trim())?;
        if cells.is_empty() {
            return Err(FormulaError::NoData);
        }
        let sum: f64 = cells.iter().map(|c| c.value.as_i8() as f64).sum();
        Ok(sum / cells.len() as f64)
    }

    /// COUNT(range) — count of cells.
    fn count(&self, args: &str) -> FormulaResult {
        let cells = self.parse_range_cells(args.trim())?;
        Ok(cells.len() as f64)
    }

    /// Helper: get (row, col) positions from a range string.
    fn get_range_positions(&self, range_str: &str) -> Result<Vec<(usize, usize)>, FormulaError> {
        let range_str = range_str.trim();

        if range_str.contains(':') {
            let parts: Vec<&str> = range_str.splitn(2, ':').collect();
            let start = parts[0].trim();
            let end = parts[1].trim();

            if start.chars().all(|c| c.is_ascii_alphabetic())
                && end.chars().all(|c| c.is_ascii_alphabetic())
            {
                let col_s = Grid::parse_cell_ref(&format!("{}1", start));
                let col_e = Grid::parse_cell_ref(&format!("{}{}", end, self.grid.rows()));
                match (col_s, col_e) {
                    (Some((_, cs)), Some((_, ce))) if cs == ce => {
                        Ok((0..self.grid.rows()).map(|r| (r, cs)).collect())
                    }
                    _ => Err(FormulaError::InvalidRange(range_str.into())),
                }
            } else {
                let start_pos = Grid::parse_cell_ref(start)
                    .ok_or_else(|| FormulaError::InvalidRange(start.into()))?;
                let end_pos = Grid::parse_cell_ref(end)
                    .ok_or_else(|| FormulaError::InvalidRange(end.into()))?;
                let (r1, r2) = (start_pos.0.min(end_pos.0), start_pos.0.max(end_pos.0));
                let (c1, c2) = (start_pos.1.min(end_pos.1), start_pos.1.max(end_pos.1));
                let mut positions = Vec::new();
                for r in r1..=r2 {
                    for c in c1..=c2 {
                        if r < self.grid.rows() && c < self.grid.cols() {
                            positions.push((r, c));
                        }
                    }
                }
                Ok(positions)
            }
        } else {
            let pos = Grid::parse_cell_ref(range_str)
                .ok_or_else(|| FormulaError::InvalidRange(range_str.into()))?;
            if pos.0 < self.grid.rows() && pos.1 < self.grid.cols() {
                Ok(vec![pos])
            } else {
                Err(FormulaError::InvalidRange(range_str.into()))
            }
        }
    }
}
