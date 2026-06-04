#[cfg(test)]
mod tests {
    use crate::*;

    // === Cell tests ===

    #[test]
    fn test_ternary_values() {
        assert_eq!(TernaryValue::Negative.as_i8(), -1);
        assert_eq!(TernaryValue::Neutral.as_i8(), 0);
        assert_eq!(TernaryValue::Positive.as_i8(), 1);
    }

    #[test]
    fn test_ternary_from_i8() {
        assert_eq!(TernaryValue::from_i8(-5), TernaryValue::Negative);
        assert_eq!(TernaryValue::from_i8(0), TernaryValue::Neutral);
        assert_eq!(TernaryValue::from_i8(42), TernaryValue::Positive);
    }

    #[test]
    fn test_ternary_flip() {
        assert_eq!(TernaryValue::Negative.flip(), TernaryValue::Positive);
        assert_eq!(TernaryValue::Positive.flip(), TernaryValue::Negative);
        assert_eq!(TernaryValue::Neutral.flip(), TernaryValue::Neutral);
    }

    #[test]
    fn test_cell_new() {
        let cell = Cell::new(TernaryValue::Positive);
        assert_eq!(cell.value, TernaryValue::Positive);
        assert_eq!(cell.fitness, 0.0);
        assert_eq!(cell.history.len(), 0);
        assert_eq!(cell.generation, 0);
    }

    #[test]
    fn test_cell_set_value_tracks_history() {
        let mut cell = Cell::new(TernaryValue::Neutral);
        cell.set_value(TernaryValue::Positive);
        assert_eq!(cell.value, TernaryValue::Positive);
        assert_eq!(cell.history, vec![TernaryValue::Neutral]);
        assert_eq!(cell.generation, 1);
    }

    #[test]
    fn test_cell_mutation_count() {
        let mut cell = Cell::new(TernaryValue::Neutral);
        assert_eq!(cell.mutation_count(), 0);
        cell.set_value(TernaryValue::Positive);
        cell.set_value(TernaryValue::Negative);
        assert_eq!(cell.mutation_count(), 2);
    }

    #[test]
    fn test_cell_reset() {
        let mut cell = Cell::new(TernaryValue::Positive);
        cell.set_fitness(5.0);
        cell.reset();
        assert_eq!(cell.value, TernaryValue::Neutral);
        assert_eq!(cell.fitness, 0.0);
        assert_eq!(cell.generation, 1);
    }

    #[test]
    fn test_cell_default_fitness() {
        let cell = Cell::new(TernaryValue::Positive);
        let fitness = cell.compute_default_fitness();
        assert!(fitness > 0.0, "Positive cell should have positive base fitness");
    }

    // === Grid tests ===

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(3, 4);
        assert_eq!(grid.dimensions(), (3, 4));
        assert_eq!(grid.len(), 12);
        assert!(!grid.is_empty());
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid = Grid::new(2, 2);
        grid.set(0, 0, TernaryValue::Positive);
        grid.set(1, 1, TernaryValue::Negative);
        assert_eq!(grid.get(0, 0).unwrap().value, TernaryValue::Positive);
        assert_eq!(grid.get(1, 1).unwrap().value, TernaryValue::Negative);
        assert_eq!(grid.get(0, 1).unwrap().value, TernaryValue::Neutral);
    }

    #[test]
    fn test_grid_range() {
        let mut grid = Grid::new(4, 4);
        grid.set(1, 1, TernaryValue::Positive);
        grid.set(2, 2, TernaryValue::Negative);
        let cells = grid.range(1, 1, 2, 2);
        assert_eq!(cells.len(), 4);
    }

    #[test]
    fn test_grid_column() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 1, TernaryValue::Positive);
        grid.set(2, 1, TernaryValue::Negative);
        let col = grid.col(1).unwrap();
        assert_eq!(col.len(), 3);
        assert_eq!(col[0].value, TernaryValue::Positive);
        assert_eq!(col[2].value, TernaryValue::Negative);
    }

    #[test]
    fn test_parse_cell_ref() {
        assert_eq!(Grid::parse_cell_ref("A1"), Some((0, 0)));
        assert_eq!(Grid::parse_cell_ref("B3"), Some((2, 1)));
        assert_eq!(Grid::parse_cell_ref("AA1"), Some((0, 26)));
        assert_eq!(Grid::parse_cell_ref("invalid"), None);
        assert_eq!(Grid::parse_cell_ref("A0"), None);
    }

    #[test]
    fn test_grid_compute_all_fitness() {
        let mut grid = Grid::new(2, 2);
        grid.set(0, 0, TernaryValue::Positive);
        grid.set(1, 1, TernaryValue::Negative);
        grid.compute_all_fitness();
        assert!(grid.get(0, 0).unwrap().fitness > 0.0);
        assert!(grid.get(1, 1).unwrap().fitness < 0.0);
    }

    // === Formula tests ===

    #[test]
    fn test_formula_sum() {
        let mut grid = Grid::new(3, 1);
        grid.set(0, 0, TernaryValue::Positive);
        grid.set(1, 0, TernaryValue::Neutral);
        grid.set(2, 0, TernaryValue::Negative);
        let mut engine = FormulaEngine::new(grid);
        let result = engine.evaluate("=SUM(A1:A3)").unwrap();
        assert_eq!(result, 0.0); // +1 + 0 + (-1) = 0
    }

    #[test]
    fn test_formula_avg() {
        let mut grid = Grid::new(2, 1);
        grid.set(0, 0, TernaryValue::Positive);
        grid.set(1, 0, TernaryValue::Positive);
        let mut engine = FormulaEngine::new(grid);
        let result = engine.evaluate("=AVG(A1:A2)").unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_formula_count() {
        let grid = Grid::new(5, 1);
        let mut engine = FormulaEngine::new(grid);
        let result = engine.evaluate("=COUNT(A1:A5)").unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_formula_entropy() {
        let mut grid = Grid::new(3, 1);
        grid.set(0, 0, TernaryValue::Positive);
        grid.set(1, 0, TernaryValue::Neutral);
        grid.set(2, 0, TernaryValue::Negative);
        let mut engine = FormulaEngine::new(grid);
        let result = engine.evaluate("=ENTROPY(A1:A3)").unwrap();
        // Max entropy for 3 equally likely values = log2(3) ≈ 1.585
        assert!((result - (3.0_f64.ln() / std::f64::consts::LN_2)).abs() < 0.01);
    }

    #[test]
    fn test_formula_unknown_function() {
        let grid = Grid::new(1, 1);
        let mut engine = FormulaEngine::new(grid);
        let err = engine.evaluate("=UNKNOWN(A1)").unwrap_err();
        assert_eq!(err, FormulaError::UnknownFunction("UNKNOWN".into()));
    }

    #[test]
    fn test_formula_best() {
        let mut grid = Grid::new(3, 1);
        grid.compute_all_fitness();
        grid.get_mut(0, 0).unwrap().set_fitness(1.0);
        grid.get_mut(1, 0).unwrap().set_fitness(3.5);
        grid.get_mut(2, 0).unwrap().set_fitness(2.0);
        let mut engine = FormulaEngine::new(grid);
        let result = engine.evaluate("=BEST(A1:A3)").unwrap();
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_formula_species() {
        let mut grid = Grid::new(6, 1);
        grid.set(0, 0, TernaryValue::Positive);
        grid.set(1, 0, TernaryValue::Positive);
        grid.set(2, 0, TernaryValue::Negative);
        grid.set(3, 0, TernaryValue::Negative);
        grid.set(4, 0, TernaryValue::Positive);
        grid.set(5, 0, TernaryValue::Neutral);
        let mut engine = FormulaEngine::new(grid);
        let result = engine.evaluate("=SPECIES(A1:A6)").unwrap();
        // ++, --, + = 3 species
        assert_eq!(result, 3.0);
    }

    // === Sort tests ===

    #[test]
    fn test_sort_rows_by_fitness() {
        let mut grid = Grid::new(3, 2);
        grid.get_mut(0, 0).unwrap().set_fitness(1.0);
        grid.get_mut(0, 1).unwrap().set_fitness(1.0);
        grid.get_mut(1, 0).unwrap().set_fitness(5.0);
        grid.get_mut(1, 1).unwrap().set_fitness(5.0);
        grid.get_mut(2, 0).unwrap().set_fitness(3.0);
        grid.get_mut(2, 1).unwrap().set_fitness(3.0);

        sort_by_fitness(&mut grid, SortAxis::Row);

        // Row 1 (fitness=10) should be first
        assert_eq!(grid.get(0, 0).unwrap().fitness, 5.0);
        assert_eq!(grid.get(1, 0).unwrap().fitness, 3.0);
        assert_eq!(grid.get(2, 0).unwrap().fitness, 1.0);
    }

    // === Autofill tests ===

    #[test]
    fn test_autofill_mutate() {
        let mut grid = Grid::new(5, 5);
        grid.set(0, 0, TernaryValue::Positive);
        let config = MutationConfig {
            mutation_rate: 0.0, // No mutations
            allow_flip: false,
            seed: Some(0),
        };
        let mutated = autofill_mutate(&mut grid, 0, 0, 0, 0, 4, 4, &config);
        assert_eq!(mutated, 0);
        // All cells should be Positive (copied from source)
        for r in 0..5 {
            for c in 0..5 {
                if r == 0 && c == 0 { continue; }
                assert_eq!(grid.get(r, c).unwrap().value, TernaryValue::Positive);
            }
        }
    }

    #[test]
    fn test_autofill_with_mutation() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, TernaryValue::Positive);
        let config = MutationConfig {
            mutation_rate: 1.0, // All mutate
            allow_flip: true,
            seed: Some(42),
        };
        let mutated = autofill_mutate(&mut grid, 0, 0, 0, 0, 2, 2, &config);
        assert!(mutated > 0, "Should have mutations");
    }

    // === Format tests ===

    #[test]
    fn test_conditional_format_colors() {
        let mut cell_green = Cell::new(TernaryValue::Positive);
        cell_green.set_fitness(1.0);
        assert_eq!(conditional_format(&cell_green), FitnessColor::Green);

        let mut cell_red = Cell::new(TernaryValue::Negative);
        cell_red.set_fitness(-1.0);
        assert_eq!(conditional_format(&cell_red), FitnessColor::Red);

        let mut cell_yellow = Cell::new(TernaryValue::Neutral);
        cell_yellow.set_fitness(0.0);
        assert_eq!(conditional_format(&cell_yellow), FitnessColor::Yellow);
    }

    #[test]
    fn test_format_cells() {
        let cells = vec![
            {
                let mut c = Cell::new(TernaryValue::Positive);
                c.set_fitness(2.0);
                c
            },
            {
                let mut c = Cell::new(TernaryValue::Neutral);
                c.set_fitness(0.0);
                c
            },
        ];
        let colors = format_cells(&cells);
        assert_eq!(colors[0], FitnessColor::Green);
        assert_eq!(colors[1], FitnessColor::Yellow);
    }

    // === Heatmap tests ===

    #[test]
    fn test_fitness_heatmap() {
        let mut grid = Grid::new(2, 2);
        grid.get_mut(0, 0).unwrap().set_fitness(0.0);
        grid.get_mut(0, 1).unwrap().set_fitness(1.0);
        grid.get_mut(1, 0).unwrap().set_fitness(0.5);
        grid.get_mut(1, 1).unwrap().set_fitness(0.75);
        let hm = fitness_heatmap(&grid);
        assert_eq!(hm.len(), 2);
        assert_eq!(hm[0].len(), 2);
        // Min (0.0) → 0.0, Max (1.0) → 1.0
        assert!((hm[0][0] - 0.0).abs() < f64::EPSILON);
        assert!((hm[0][1] - 1.0).abs() < f64::EPSILON);
        assert!((hm[1][0] - 0.5).abs() < f64::EPSILON);
        assert!((hm[1][1] - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_heatmap_uniform_fitness() {
        let mut grid = Grid::new(2, 2);
        grid.compute_all_fitness(); // All neutral → same fitness
        let hm = fitness_heatmap(&grid);
        // All should be 0.5 (uniform)
        for row in &hm {
            for &val in row {
                assert!((val - 0.5).abs() < f64::EPSILON);
            }
        }
    }

    #[test]
    fn test_ternary_display() {
        assert_eq!(format!("{}", TernaryValue::Negative), "-1");
        assert_eq!(format!("{}", TernaryValue::Neutral), "0");
        assert_eq!(format!("{}", TernaryValue::Positive), "+1");
    }

    #[test]
    fn test_ternary_from_seed() {
        assert_eq!(TernaryValue::from_seed(0), TernaryValue::Negative);
        assert_eq!(TernaryValue::from_seed(1), TernaryValue::Neutral);
        assert_eq!(TernaryValue::from_seed(2), TernaryValue::Positive);
        // Wraps
        assert_eq!(TernaryValue::from_seed(3), TernaryValue::Negative);
    }
}
