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

/// Lambertian Material.
pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn shade(&self, input: MatInput) -> Option<ShadeOutput> {
        let mut scatter_dir = input.surface_norm + Vec3::random_unit();
        
        // prevent scatter_dir nearly equal vec3(0.0).
        if scatter_dir.length_square() < 3e-16 {
            scatter_dir = input.surface_norm;
        }

        let scatter = Ray::new(input.hitted_position, scatter_dir);

        Some(ShadeOutput {
            scatter, 
            attenuation: self.albedo
        })
    }
}

/// Emissive Material.
pub struct Emissive {
    pub emissive: Vec3
}

impl Emissive {
    pub fn new(emissive: Vec3) -> Emissive {
        Emissive { emissive }
    }
}

impl Material for Emissive {
    fn emissive(&self, _input: MatInput) -> Vec3 {
        self.emissive
    }

    fn shade(&self, _input: MatInput) -> Option<ShadeOutput> {
        None
    }
}