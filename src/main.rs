use std::{ env, process };
use raytracing::{ 
    utils::{ cfg_loader, img_saver },
    Renderer, RendererConfig,
    Camera, 
};

fn main() {
    let usage = "usage: raytracing <cfg>";
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("error: no .cfg file provided!");
        eprintln!("{}", usage);
        process::exit(-1);
    }
    else if args.len() > 2 {
        eprintln!("error: too many arguments!");
        eprintln!("{}", usage);
        process::exit(-1);
    }

    print!("Loading configuration from \"{}\" ... ", &args[1]);
    let mut config = cfg_loader::from_file(&args[1]);
    println!("finished.");

    if !config.comments.is_empty() {
        println!("\n{}", &config.comments);
    }

    println!("Target Information:");
    println!("> name: {}", config.target_name);
    println!("> pixel: {:#?}", config.target_pixel);
    println!("> resolution: {:?}\n", config.target_resolution);

    println!("Renderer Features:");
    println!("> backend: {:#?}", config.renderer_backend);
    println!("> BVH acceleration: {}", config.renderer_bvh_acc);
    println!("> Max bounce depth: {}", config.renderer_max_depth);
    println!("> Sample per pixel: {}\n", config.renderer_spp);

    println!("Scene entities: {}\n", config.scene.as_ref().unwrap().entities.len());

    let mut renderer = Renderer::new(
        RendererConfig {
            backend: config.renderer_backend,
            max_depth: config.renderer_max_depth,
            spp: config.renderer_spp
        }
    );

    let scene = config.scene.take().unwrap();

    let camera = Camera::new(
        config.camera_pos,
        config.camera_dir,
        config.camera_viewport,
        config.camera_fov
    );

    println!("Rendering scene...");
    let framebuffer = renderer.render(&scene, &camera, config.target_resolution);

    println!("\nCopying buffer...");
    img_saver::save_as(
        &config.target_name, 
        &framebuffer, 
        config.target_pixel, 
        config.target_resolution.0, 
        config.target_resolution.1
    ).unwrap();

    println!("Done.");
}