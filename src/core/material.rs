use super::{ Vec3, Ray };
 
/// Material's input data-type.
#[derive(Clone, Copy)]
pub struct MatInput {
    /// Incident ray.
    pub incident_ray: Ray,
    /// The normal vector, whose direction always same as `incident_ray`.
    /// 
    /// aka. `dot(incident_ray, surface_norm) > 0`
    pub surface_norm: Vec3,
    /// Whether the ray hits from the outside surface.
    pub surface_front: bool,
    /// The position where the ray hits on the surface.
    pub hitted_position: Vec3
}

/// Material's shading output data-type.
pub struct ShadeOutput {
    /// The scattered ray after shading.
    pub scatter: Ray,
    /// The attenuation contributed by the surface.
    pub attenuation: Vec3
}

/// Abstraction for material.
pub trait Material {
    /// Get the surface emissive color of emissive material. (*optional*)
    /// 
    /// Return `(0.0, 0.0, 0.0)` by default.
    fn emissive(&self, _input: MatInput) -> Vec3 {
        Vec3::from_scalar(0.0)
    }

    /// Shading the surface information.
    fn shade(&self, _input: MatInput) -> Option<ShadeOutput> {
        None
    }
}