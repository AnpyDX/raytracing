use super::math::Vec3;

/// Abstraction of a physical ray.
#[derive(Clone)]
pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3
}

impl Ray {
    /// Create a new ray object.
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { ori: origin, dir: direction }
    }

    /// Get ray reaching point with given steps.
    pub fn position(&self, step: f64) -> Vec3 {
        self.ori + self.dir * step
    }
}