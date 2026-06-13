# Ternary Spreadsheet — Logic Engine with Three-Valued Cells

**Ternary Spreadsheet** is a spreadsheet engine where cells can hold ternary values {-1, 0, +1} in addition to numbers and text. It supports formula evaluation with ternary logic operators (AND, OR, NOT across cell ranges), enabling three-valued reasoning in a familiar grid interface.

## Why It Matters

Spreadsheets are the most widely used programming paradigm in the world — everyone from accountants to scientists uses them. Adding ternary logic to spreadsheets brings three-valued reasoning to a tool people already know. This is particularly valuable for decision matrices: instead of binary yes/no evaluations, cells can represent "approved (1), needs review (0), rejected (-1)" — enabling nuanced decision workflows. Ternary AND across a range computes the minimum (most restrictive opinion), and ternary OR computes the maximum (most permissive opinion). For fleet management dashboards, this means a ternary spreadsheet can aggregate agent health signals into fleet-level assessments without custom code.

## How It Works

### Cell Types

Each cell holds one of:
- `Tri(i8)`: Ternary value {-1, 0, +1}
- `Number(f64)`: Floating-point value
- `Text(String)`: String content
- `Empty`: No value

### Ternary Logic Operations

**AND across cells**: Takes the minimum of all ternary values. In Kleene logic, AND is pessimistic: if any cell is -1, the result is -1; if any is 0, the result is 0 (unless one is -1); only all +1 yields +1.

```
ternary_and(cells) = min(all Tri values)
```

**OR across cells**: Takes the maximum. Kleene OR is optimistic: if any cell is +1, the result is +1.

```
ternary_or(cells) = max(all Tri values)
```

Both are O(c) for c cells in the range.

### Formula Evaluation

Formulas are stored as strings and evaluated by referencing other cells. The formula engine supports:
- Cell references (e.g., `A1`, `B3`)
- Ternary operators (AND, OR, NOT)
- Arithmetic on Number cells
- Range operations (e.g., `AND(A1:A10)`)

### Spreadsheet Model

The `Spreadsheet` type manages a `HashMap<(row, col), Cell>` with configurable dimensions. Cell access is O(1) average. Empty cells return `CellValue::Empty`.

## Quick Start

```rust
use ternary_spreadsheet::{Spreadsheet, CellValue};

let mut ss = Spreadsheet::new(10, 10);

// Set ternary values
ss.set(0, 0, CellValue::Tri(1));   // Approved
ss.set(1, 0, CellValue::Tri(0));   // Needs review
ss.set(2, 0, CellValue::Tri(-1));  // Rejected

// Ternary AND across range
let result = ss.ternary_and(&[(0, 0), (1, 0), (2, 0)]);
assert_eq!(result, -1); // Most restrictive opinion

// Set formula
ss.set_formula(3, 0, "AND(A1:A3)".to_string());
```

```bash
cargo add ternary-spreadsheet
```

## API

| Type / Function | Description |
|---|---|
| `CellValue` | `Tri(i8)`, `Number(f64)`, `Text(String)`, `Empty` |
| `Cell` | Value + optional formula |
| `Spreadsheet` | `new(rows, cols)`, `get()`, `set()`, `set_formula()`, `ternary_and()` |

## Architecture Notes

The spreadsheet is the human interface to **SuperInstance** fleet state. Operators view agent decisions as ternary values in a grid: +1 (healthy), 0 (unknown), -1 (failed). The γ + η = C conservation is reflected in the aggregate: the ternary AND of all agents gives the fleet-level health assessment. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Klir, George & Yuan, Bo. *Fuzzy Sets and Fuzzy Logic*, Prentice Hall, 1995 — three-valued logic.
| Codd, E. F. "Extending the Database Relational Model to Capture More Meaning," *ACM TODS*, 4(4), 1979 — SQL NULL semantics.
| Sipser, Michael. *Introduction to the Theory of Computation*, 3rd ed., Cengage, 2013.

## License

MIT
