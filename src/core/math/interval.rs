use std::f64::INFINITY;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    /// `min <= value <= max`
    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    /// `min < value < max`
    pub fn surrounds(&self, value: f64) -> bool {
        value > self.min && value < self.max
    }

    /// Clamp value in interval.
    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.min { return self.min }
        else if value > self.max { return self.max }
        else { return value }
    }

    /// Generate a empty interval.
    pub fn empty() -> Self {
        Interval { min: INFINITY, max: -INFINITY }
    }

    /// Generate a universe interval.
    pub fn universe() -> Self {
        Interval { min: -INFINITY, max: INFINITY }
    }
}