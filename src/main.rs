use std::rc::Rc;
use raytracing::utils::png;
use raytracing::{
    Camera, Scene,
    Renderer, RendererConfig, BackendConfig, DebugLevel,
    Entity, Sphere,
    Material, MatInput, Lambertian, 
    Vec3, Vec2, radians
};

struct Background;

impl Material for Background {
    fn emissive(&self, input: MatInput) -> Vec3 {
        let unit_dir = input.incident_ray.dir.normalized();
        let a = 0.5 * (unit_dir.y + 1.0);
        return (1.0 - a) * Vec3::from_scalar(1.0) + a * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn copy_buffer(src: &Vec<Vec3>, dst: &mut png::Image) {
    for pixel in src {
        dst.write()
    }
}

fn main() {
    let camera = Camera::new(
        Vec3::from_scalar(0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(1.0, 1.0),
        radians(45.0)
    );

    let diffuse_mat: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(1.0, 1.0, 1.0)));
    let bg_mat = Rc::new(Background);
    
    let mut scene = Scene::new(bg_mat);
    scene.add(Entity { 
        mesh: Rc::new(
            Sphere {
                center: Vec3::new(0.0, 0.0, 1.0),
                radius: 0.5   
            }),
        mat: diffuse_mat
     });

    let render_config = RendererConfig {
        backend: BackendConfig::CPUDrivenS,
        dbg_level: DebugLevel::Full,
        max_depth: 64,
        spp: 64
    };
    let mut renderer = Renderer::new(render_config);

    let output = renderer.render(&scene, &camera, (500, 500));


}
