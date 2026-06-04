# Future Integration: ternary-spreadsheet

## Current State

ternary-spreadsheet implements core logic for the SuperInstance Spreadsheet where every cell is a ternary intelligence. `Cell` holds `TernaryValue` with fitness. `Grid` manages the cell matrix. `FormulaEngine` evaluates spreadsheet formulas with `FormulaResult` and `FormulaError`. `sort_by_fitness()` with `SortAxis` implements natural selection sorting. `autofill_mutate()` with `MutationConfig` introduces variation. `conditional_format()` and `conditional_format_with_thresholds()` apply visual formatting. `fitness_heatmap()` generates heat map representations.

## Integration Opportunities

### Cells as Rooms / Agents (Primary Integration)

The spreadsheet IS the world model. From the Cross-Pollination Report's "Spreadsheet-as-World" thesis:

- **Cell = Room**: Each `Cell` IS a PLATO room. `TernaryValue` = room state (Avoid/Explore/Choose). `fitness` = room health. The `FormulaEngine` evaluates each cell's "physics" — formulas are the laws of the cell-universe.
- **`sort_by_fitness()` = Natural Selection**: Sorting cells by fitness IS evolution. High-fitness cells rise, low-fitness cells sink. Combined with `autofill_mutate()`, this is a complete evolutionary cycle: evaluate → sort → mutate → repeat.
- **`autofill_mutate()` = Genetic Variation**: `MutationConfig` controls the mutation rate and magnitude. This maps to `ternary-evolution`'s crossover/mutation operators applied across the spreadsheet grid.

### ternary-cell → Living Spreadsheet Cells

Replace static `Cell` with `ternary-cell::TernaryCell`. Each spreadsheet cell becomes a living agent with the six-phase tick cycle:

1. **Predict** = cell's cached formula result (expected value)
2. **Perceive** = `FormulaEngine` evaluates actual value
3. **Surprise** = difference between predicted and actual
4. **Vibe** = `conditional_format()` — cell color = surprise magnitude
5. **GC** = `sort_by_fitness()` removes low-energy cells
6. **Conservation** = column sum invariants

### ternary-world → Spreadsheet Physics Engine

`ternary-world::WorldGrid` IS the spreadsheet grid. `WorldPhysics::apply()` IS the formula engine's conservation law: when cell A1 changes, neighboring cells compensate. `WorldObserver` tracks column metrics. `WorldSnapshot` = save/load spreadsheet state.

### =EVOLVE() Formula

The killer formula: `=EVOLVE(B2:B50, 100)` runs 100 generations of evolution on the range. Each generation:

1. `FormulaEngine` evaluates all cells (perceive)
2. `sort_by_fitness()` ranks them (selection)
3. `autofill_mutate()` introduces variation (mutation)
4. Conservation law maintains population size (energy budget)

This IS natural selection in a spreadsheet. The user sees evolution happening in real-time.

## Potential in Mature Systems

ternary-spreadsheet becomes the primary user-facing product of the ecosystem. A browser-based spreadsheet (BrowserRoom via WASM) where cells are alive, formulas are physics, and sorting is natural selection. The "One Strategy, Three Brains" demo: same spreadsheet on ESP32 (single cell, lookup table), Pi (row of cells, formula evaluation), DGX (full grid, GPU-accelerated evolution). Same intelligence, three hardware bodies.

## Cross-Pollination Ideas

- **Spreadsheet → PLATO tile store**: Each cell's formula history is a tile. When a cell's surprise is high, it generates a tile for PLATO: "Cell C7 predicted 0.5, got 0.1, surprise=0.4."
- **Spreadsheet → fleet dashboard**: A fleet of agents displayed as a spreadsheet. Each row = agent, columns = metrics. Sort by fitness = natural fleet selection.
- **conditional_format() → ternary-attention visualization**: Highlight cells that "attend" to each other. Cells with high mutual information share a color.

## Dependencies for Next Steps

1. `Cell` → `TernaryCell` migration (static → living cells)
2. `FormulaEngine` → `WorldPhysics` bridge
3. `=EVOLVE()` formula implementation
4. WASM compilation for browser-based BrowserRoom
5. `Grid` → `CellGrid` overlay for agent-environment coupling
