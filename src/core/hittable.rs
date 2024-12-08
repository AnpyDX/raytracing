use super::{ Ray, math::{ Vec3, Interval } };

/// Hitting information
pub struct HittingInfo {
    /// The hitted position on hittable.
    pub position: Vec3,
    /// The normal(normalized) of the hitted surface.
    /// 
    /// **NOTE**
    /// 
    /// In order to facilite color caculation, 
    /// normal's direction will be inversed if neccearry 
    /// 
    /// to make it **always** same as ray's direction.
    /// (aka. `normal.dot(ray.dir) > 0.0`)
    pub normal: Vec3,
    /// The step of ray when surface is hitted.
    pub step: f64,
    /// Whether the ray hit the front face.
    pub is_front: bool
}

/// Hittable object abstraction
pub trait Hittable {
    /// Caculate whether object is hitted.
    /// 
    /// If ray did not hit the object, return `None`.
    /// 
    /// - `step_limit` the interval for ray's step.
    fn hit(&self, ray: &Ray, step_limit: Interval) -> Option<HittingInfo>;
}