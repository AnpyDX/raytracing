// Prefab Shapes

use super::{ 
    Hittable, SurfaceInfo, Ray, 
    math::{ Vec3, Interval } 
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Vec3
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, color: Vec3) -> Self {
        Sphere { center, radius, color }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, step_limit: Interval) -> Option<SurfaceInfo> {
        let oc = self.center - ray.ori;
        let a = ray.dir.dot(ray.dir);
        let h = ray.dir.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        
        if discriminant < 0.0 {
            return None;
        }

        let mut step = (h - discriminant.sqrt()) / a;
        if !step_limit.surrounds(step) {
            step = (h + discriminant.sqrt()) / a;
            if !step_limit.surrounds(step) {
                return None;
            }
        }

        let point = ray.position(step);

        let mut is_front = true;
        let mut normal = (point - self.center).normalized();
        if normal.dot(ray.dir) > 0.0 {
            is_front = false;
            normal = -normal;
        }

        Some(SurfaceInfo {
            point, normal, step, is_front
        })
    }
}