use super::{ Ray, math::{ Vec3, Interval } };

/// Hitted surface info
pub struct SurfaceInfo {
    /// The hitted point on hittable.
    pub point: Vec3,
    /// The normal(normalized) of the hitted surface.
    /// 
    /// **NOTE**
    /// 
    /// In order to facilite color caculation, 
    /// normal's direction will be inversed if neccearry 
    /// 
    /// to make it same as Ray's direction.
    /// 
    /// That is `normal.dot(ray.dir) > 0.0`.
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
    /// - `rmin` and `rmax` are the minimum and maximum step of ray.
    fn hit(&self, ray: &Ray, step_limit: Interval) -> Option<SurfaceInfo>;
}