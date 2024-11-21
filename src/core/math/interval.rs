use super::INF;

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
}

pub const EMPTY_INTERVAL: Interval = Interval { min: INF, max: -INF };
pub const UNIVERSE_INTERVAL: Interval = Interval { min: -INF, max: INF };