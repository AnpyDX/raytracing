use super::{ 
    Hittable, SurfaceInfo, Ray,
    math::Interval
};

// TODO add entiy system to support delete operation
pub struct Scene(Vec<Box<dyn Hittable>>);

impl Scene {
    pub fn new() -> Scene {
        Scene(Vec::new())
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.0.push(hittable)
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, step_limit: Interval) -> Option<SurfaceInfo> {
        for enity in &self.0 {
            if let Some(surface) = enity.hit(ray, step_limit) {
                return Some(surface);
            }
        }

        return None;
    }
}