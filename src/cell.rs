/// A ternary value: negative, neutral, or positive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TernaryValue {
    Negative = -1,
    Neutral = 0,
    Positive = 1,
}

impl TernaryValue {
    /// Create from an i8. Clamps to the nearest ternary value.
    pub fn from_i8(v: i8) -> Self {
        match v {
            i8::MIN..=-1 => TernaryValue::Negative,
            0 => TernaryValue::Neutral,
            1..=i8::MAX => TernaryValue::Positive,
        }
    }

    /// Convert to i8.
    pub fn as_i8(self) -> i8 {
        self as i8
    }

    /// Flip the sign: Negative ↔ Positive, Neutral stays Neutral.
    pub fn flip(self) -> Self {
        match self {
            TernaryValue::Negative => TernaryValue::Positive,
            TernaryValue::Neutral => TernaryValue::Neutral,
            TernaryValue::Positive => TernaryValue::Negative,
        }
    }

    /// Get all three values.
    pub fn all() -> [TernaryValue; 3] {
        [TernaryValue::Negative, TernaryValue::Neutral, TernaryValue::Positive]
    }

    /// Random value given a u32 seed (simple deterministic).
    pub fn from_seed(seed: u32) -> Self {
        match seed % 3 {
            0 => TernaryValue::Negative,
            1 => TernaryValue::Neutral,
            _ => TernaryValue::Positive,
        }
    }
}

impl std::fmt::Display for TernaryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TernaryValue::Negative => write!(f, "-1"),
            TernaryValue::Neutral => write!(f, "0"),
            TernaryValue::Positive => write!(f, "+1"),
        }
    }
}

/// A single ternary cell with value, fitness, and history.
#[derive(Debug, Clone)]
pub struct Cell {
    pub value: TernaryValue,
    pub fitness: f64,
    pub history: Vec<TernaryValue>,
    pub generation: u64,
}

impl Cell {
    /// Create a new cell with the given value and zero fitness.
    pub fn new(value: TernaryValue) -> Self {
        Cell {
            value,
            fitness: 0.0,
            history: vec![],
            generation: 0,
        }
    }

    /// Create a neutral cell.
    pub fn neutral() -> Self {
        Self::new(TernaryValue::Neutral)
    }

    /// Set the value, recording the previous value in history.
    pub fn set_value(&mut self, value: TernaryValue) {
        self.history.push(self.value);
        self.value = value;
        self.generation += 1;
    }

    /// Set the fitness score.
    pub fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness;
    }

    /// Compute a simple fitness: value magnitude scaled by generation diversity.
    pub fn compute_default_fitness(&self) -> f64 {
        let base = self.value.as_i8() as f64;
        let unique_count = {
            let mut vals = self.history.clone();
            vals.sort();
            vals.dedup();
            vals.len() as f64
        };
        base * (1.0 + unique_count * 0.1)
    }

    /// How many times has this cell changed value?
    pub fn mutation_count(&self) -> usize {
        self.history.len()
    }

    /// Reset the cell to neutral with a new generation.
    pub fn reset(&mut self) {
        self.history.push(self.value);
        self.value = TernaryValue::Neutral;
        self.fitness = 0.0;
        self.generation += 1;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::neutral()
    }
}
