# ternary-spreadsheet

Core logic for the **SuperInstance Spreadsheet** — a familiar spreadsheet interface where every cell is a tiny ternary intelligence.

## The Concept

Imagine a spreadsheet where each cell doesn't just hold a number — it holds an opinion. Every cell is a ternary agent with a value of **−1** (avoid), **0** (unknown), or **+1** (choose). Each cell has:

- A **fitness score** — how well its value is performing
- A **history** — the trail of values it's held
- A **generation counter** — how many times it's evolved

The grid becomes a living population of ternary intelligences. You can evolve them, sort by fitness, mutate ranges, and watch natural selection shape the spreadsheet in real time.

## Features

### 🧬 Cell
A ternary cell with value `{−1, 0, +1}`, fitness score, and mutation history.

### 📊 Grid
A 2D grid of cells with row/column operations, cell references (`A1`, `B3`), and range queries.

### ⚡ Formula Engine
Evaluate spreadsheet formulas with AI-powered functions:

| Formula | Description |
|---------|-------------|
| `=EVOLVE(A1:A10, 100)` | Run 100 generations of evolution on a range |
| `=BEST(B:B)` | Find the highest fitness in column B |
| `=SPECIES(C1:C50)` | Count distinct species (clusters of same-sign values) |
| `=EXHAUSTIVE(D1:D4)` | Try all 3⁴=81 combinations, return best total fitness |
| `=ENTROPY(E1:E20)` | Compute Shannon entropy of ternary values |
| `=SUM(A1:A5)` | Sum cell values |
| `=AVG(A1:A5)` | Average cell values |
| `=COUNT(A1:A5)` | Count cells in range |

### 🔄 Sort by Fitness
Sort rows or columns by fitness — natural selection for your spreadsheet. Fittest rows rise to the top.

### 🎲 Autofill with Mutation
Like regular autofill, but with controlled randomness. Copy a cell's value across a range with a configurable mutation rate. Some cells flip, some go neutral — evolution in action.

### 🎨 Conditional Format
Assign colors based on fitness:
- 🟢 **Green** — positive fitness (choose)
- 🟡 **Yellow** — near zero (unknown)
- 🔴 **Red** — negative fitness (avoid)

### 🗺️ Fitness Heatmap
Compute normalized heatmap values `[0.0, 1.0]` for the entire grid — visualize which regions are thriving.

## Usage

```rust
use ternary_spreadsheet::*;

// Create a 10x10 grid
let mut grid = Grid::new(10, 10);

// Set some values
grid.set(0, 0, TernaryValue::Positive);
grid.set(1, 0, TernaryValue::Negative);

// Compute fitness for all cells
grid.compute_all_fitness();

// Use the formula engine
let mut engine = FormulaEngine::new(grid);
let best = engine.evaluate("=BEST(A:A)").unwrap();

// Sort rows by fitness (fittest first)
sort_by_fitness(engine.grid_mut(), SortAxis::Row);

// Get conditional colors
let color = conditional_format(engine.grid().get(0, 0).unwrap());

// Generate heatmap
let heatmap = fitness_heatmap(engine.grid());
```

## Design Principles

- **Pure Rust** — no unsafe code, no external dependencies
- **Zero-cost abstractions** — enum-based ternary values compile to efficient code
- **Composable** — mix and match operations like building blocks
- **Deterministic** — seeded mutations for reproducible experiments

## License

MIT
