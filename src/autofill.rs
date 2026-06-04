use crate::cell::TernaryValue;
use crate::grid::Grid;

/// Configuration for autofill mutation.
#[derive(Debug, Clone)]
pub struct MutationConfig {
    /// Probability of mutation per cell (0.0 - 1.0).
    pub mutation_rate: f64,
    /// If true, mutations can flip sign; otherwise they tend toward neutral.
    pub allow_flip: bool,
    /// Seed for deterministic behavior (None = use counter).
    pub seed: Option<u64>,
}

impl Default for MutationConfig {
    fn default() -> Self {
        MutationConfig {
            mutation_rate: 0.3,
            allow_flip: true,
            seed: None,
        }
    }
}

/// Autofill a range with mutation. Takes the source cell's value and propagates it
/// across the target range, applying random mutations based on the config.
/// Returns the number of cells mutated.
pub fn autofill_mutate(
    grid: &mut Grid,
    src_row: usize,
    src_col: usize,
    dst_r1: usize,
    dst_c1: usize,
    dst_r2: usize,
    dst_c2: usize,
    config: &MutationConfig,
) -> usize {
    let source = grid.get(src_row, src_col)
        .map(|c| c.value)
        .unwrap_or(TernaryValue::Neutral);

    let (r_min, r_max) = (dst_r1.min(dst_r2), dst_r1.max(dst_r2));
    let (c_min, c_max) = (dst_c1.min(dst_c2), dst_c1.max(dst_c2));

    let mut mutated = 0usize;
    let mut counter: u64 = config.seed.unwrap_or(42);

    for r in r_min..=r_max {
        for c in c_min..=c_max {
            // Skip source cell
            if r == src_row && c == src_col {
                continue;
            }

            counter = counter.wrapping_mul(6364136223846793005).wrapping_add(1);
            let rand_val = (counter >> 33) as f64 / (1u64 << 31) as f64;

            let new_value = if rand_val < config.mutation_rate {
                mutated += 1;
                if config.allow_flip {
                    counter = counter.wrapping_mul(6364136223846793005).wrapping_add(1);
                    let flip_val = (counter >> 33) as u32;
                    TernaryValue::from_seed(flip_val)
                } else {
                    TernaryValue::Neutral
                }
            } else {
                source
            };

            if let Some(cell) = grid.get_mut(r, c) {
                cell.set_value(new_value);
            }
        }
    }

    mutated
}
