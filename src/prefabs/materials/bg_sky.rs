use super::super::super::core::{
    Material, MatInput, Vec3
};

pub struct BgSky;

impl Material for BgSky {
    fn emissive(&self, input: MatInput) -> Vec3 {
        let unit_dir = input.incident_ray.dir.normalized();
        let a = 0.5 * (unit_dir.y + 1.0);
        return (1.0 - a) * Vec3::from_scalar(1.0) + a * Vec3::new(0.5, 0.7, 1.0);
    }
}