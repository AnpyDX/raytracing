use std::rc::Rc;
use image;
use raytracing::{
    Camera, Scene,
    Renderer, RendererConfig, BackendConfig, DebugLevel,
    Entity, Sphere,
    Material, MatInput, Lambertian, 
    Vec3, Vec2, Interval, radians
};

struct Background;

impl Material for Background {
    fn emissive(&self, input: MatInput) -> Vec3 {
        let unit_dir = input.incident_ray.dir.normalized();
        let a = 0.5 * (unit_dir.y + 1.0);
        return (1.0 - a) * Vec3::from_scalar(1.0) + a * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn save_as_png(name: &str, width: u32, height: u32, src: &Vec<Vec3>) {
    // mapping color.
    let linear_to_gamma = |comp: f64| -> f64 {
        if comp < 0.0 { return 0.0 }
        return comp.sqrt();
    };

    let mut buffer = vec![0; src.len() * 3];
    let intensity = Interval::new(0.0, 0.999);

    for index in 0..src.len() {
        let pixel = src[index];
        let r = (256.0 * intensity.clamp(linear_to_gamma(pixel.x))) as u8;
        let g = (256.0 * intensity.clamp(linear_to_gamma(pixel.y))) as u8;
        let b = (256.0 * intensity.clamp(linear_to_gamma(pixel.z))) as u8;
        buffer[index * 3] = r;
        buffer[index * 3 + 1] = g;
        buffer[index * 3 + 2] = b;
    }

    image::save_buffer_with_format(
        name, buffer.as_ref(), 
        width, height, 
        image::ColorType::Rgb8, image::ImageFormat::Png
    ).unwrap();
}

fn main() {
    let camera = Camera::new(
        Vec3::from_scalar(0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(0.5, 0.5),
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
        mat: Rc::clone(&diffuse_mat)
     });

     scene.add(Entity { 
        mesh: Rc::new(
            Sphere {
                center: Vec3::new(0.0,-100.5,-1.0),
                radius: 100.0
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

    save_as_png("output.png", 500, 500, &output);
}
