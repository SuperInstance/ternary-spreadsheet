# ternary-spreadsheet

A **ternary logic spreadsheet engine** where every cell holds a value in {-1, 0, +1}. Supports formula evaluation (`SUM`, `AVG`, `ENTROPY`, `EVOLVE`, `BEST`, `SPECIES`, `EXHAUSTIVE`), conditional formatting via fitness coloring, mutation-based autofill, and fitness-sorted row/column operations.

## Why It Matters

Traditional spreadsheets operate on real-valued cells, which over-encodes for ternary decision problems. When modeling agent strategies, evolutionary fitness landscapes, or balance-sheet scenarios where outcomes are strictly positive/neutral/negative, a ternary spreadsheet:

1. **Eliminates false precision** — no phantom decimals from floating-point artifacts
2. **Enables combinatorial search** — exhaustive enumeration is 3^N, not ∞^N
3. **Supports native evolution** — genetic operators work directly on {-1, 0, +1} values
4. **Computes Shannon entropy** — information-theoretic analysis of cell distributions

## How It Works

### Cell Model

Each `Cell` stores:
- `value: TernaryValue` ∈ {Negative(-1), Neutral(0), Positive(+1)}
- `fitness: f64` — computed from value and history diversity
- `history: Vec<TernaryValue>` — previous values (mutation log)
- `generation: u64` — mutation count

**Default fitness:**

```
f(cell) = value_i8 · (1 + unique_count(history) · 0.1)
```

Cells that have changed values more often get a diversity bonus.

### Grid Operations

Standard grid operations with A1-style cell references (e.g., `B3` = row 2, col 1):

- `get(row, col)`, `set(row, col, value)` — cell access
- `range(r1, c1, r2, c2)` — rectangular slice
- `col(c)`, `row(r)` — full column/row
- `compute_all_fitness()` — batch fitness evaluation

### Formula Engine

| Formula | Description |
|---------|-------------|
| `=SUM(range)` | Σ cell values as f64 |
| `=AVG(range)` | Mean cell value |
| `=COUNT(range)` | Number of cells |
| `=ENTROPY(range)` | Shannon entropy: `H = -Σ pᵢ log₂(pᵢ)` |
| `=BEST(range)` | Maximum fitness in range |
| `=SPECIES(range)` | Count contiguous same-sign clusters |
| `=EVOLVE(range, gens)` | Run N generations of evolution on range |
| `=EXHAUSTIVE(range)` | Try all 3^N combinations, return best total fitness |

### Shannon Entropy

For a range with *n* cells and counts (c₋₁, c₀, c₊₁):

```
H = - Σ (cᵢ/n) · log₂(cᵢ/n)   for cᵢ > 0
```

Maximum: `H = log₂(3) ≈ 1.585` bits (uniform distribution).

**Complexity:** O(n) per entropy computation.

### Evolution Formula

`=EVOLVE(A1:A10, 100)` runs 100 generations of genetic optimization on the range:

1. Compute fitness for all cells
2. Sort by fitness (descending)
3. Keep top 50%; replace bottom 50% with random ternary values
4. Track best fitness across all generations

**Complexity:** O(gens · n · log n).

### Exhaustive Search

`EXHAUSTIVE(A1:A3)` tries all 3³ = 27 combinations:

```
for combo in 0..3^N:
    val = combo
    for each cell:
        assign TernaryValue::from_seed(val % 3)
        val = val / 3
    compute total fitness
return best total
```

Each cell gets a different base-3 digit of `combo`, so every combination is
distinct. The search is side-effect-free: cell state (value, history,
generation) is saved before and restored after.

**Complexity:** O(3^N · N). Limited to N ≤ 10 (59,049 combinations).

### Sorting

`sort_by_fitness(grid, axis)` sorts rows or columns by total fitness (descending). Fittest rows/columns appear first.

**Complexity:** O(R log R · C) for row sort, O(C log C · R) for column sort.

## Quick Start

```rust
use ternary_spreadsheet::{Grid, FormulaEngine, TernaryValue};

let mut grid = Grid::new(3, 1);
grid.set(0, 0, TernaryValue::Positive);
grid.set(1, 0, TernaryValue::Neutral);
grid.set(2, 0, TernaryValue::Negative);

let mut engine = FormulaEngine::new(grid);

let sum = engine.evaluate("=SUM(A1:A3)").unwrap();    // 0.0
let entropy = engine.evaluate("=ENTROPY(A1:A3)").unwrap(); // ≈1.585
```

## API

| Type | Key Methods |
|------|-------------|
| `Grid` | `new(rows, cols)`, `get/set`, `range`, `col`, `compute_all_fitness()` |
| `FormulaEngine` | `evaluate(formula_str)`, `grid()`, `grid_mut()` |
| `Cell` | `set_value()`, `set_fitness()`, `compute_default_fitness()`, `reset()` |
| `TernaryValue` | `Negative`, `Neutral`, `Positive` — `from_i8()`, `flip()`, `from_seed()` |
| `sort_by_fitness(grid, axis)` | Sort rows/columns by total fitness |

## Architecture Notes

The **γ + η = C** invariant is embodied in the EVOLVE formula: *generation* (γ) is the mutation operator producing new ternary values, *entropy* (η) is the Shannon entropy of the cell-value distribution (computed by ENTROPY), and *conservation* (C) is the invariant that the grid dimensions never change — evolution reshuffles values within a fixed-size space. The EXHAUSTIVE search is the conservation enforcement: it proves that no configuration exceeds the found maximum, closing the γ-η loop.

## References

- **Spreadsheets as programming environments:** Sestoft, P. *Spreadsheet Implementation Technology* (2014)
- **Shannon entropy:** Shannon, C. E. "A Mathematical Theory of Communication" (1948)
- **Genetic algorithms on discrete spaces:** Holland, J. *Adaptation in Natural and Artificial Systems* (1975)
- **Exhaustive combinatorial search:** Knuth, D. *The Art of Computer Programming* Vol. 4A (2011)

## License

MIT
