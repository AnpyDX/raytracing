use super::{ 
    Scene, Camera, Ray, Hittable, 
    math::{ Vec3, Interval, INF }
};

pub struct Renderer;

impl Renderer {
    pub fn render(scene: &Scene, camera: &mut Camera) {
        for task in camera.iter() {
            match scene.hit(&task.ray, Interval::new(0.0, INF)) {
                Some(surface) => {
                    *task.pixel = (surface.normal + Vec3::from_scalar(1.0)) / 2.0;
                },
                None => {
                    *task.pixel = Self::get_background(&task.ray);
                }
            }
        }
    }

    fn get_background(ray: &Ray) -> Vec3 {
        let unit_dir = ray.dir.normalized();
        let a = 0.5 * (unit_dir.y + 1.0);
        (-a + 1.0) * Vec3::from_scalar(1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}