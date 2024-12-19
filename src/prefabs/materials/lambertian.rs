use super::super::super::core::{
    Material, MatInput, ShadeOutput, Ray, Vec3
};

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