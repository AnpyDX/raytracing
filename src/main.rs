use raytracing::{
    core::*, utils::png
};

fn copy_buffer(image: &mut png::Image, screen: &Screen) {
    for y in 0..image.size.1 {
        for x in 0..image.size.0 {
            let index = (y * image.size.0 + x) as usize;
            let color = screen.buffer[index];
            let color = (
                (256.0 * color.x) as u8, 
                (256.0 * color.y) as u8, 
                (256.0 * color.z) as u8);
            image.write((x, y), color);
        }
    }
}

fn main() {
    let mut output_image = png::Image::new(1920, 1080);

    let screen = Screen::new(1920, 1080);
    let mut camera = Camera::new(
        Vec3::from_scalar(0.0), 
        Vec3::new(0.0, 0.0, -1.0), 
        1.0
    );

    camera.set_screen(screen, 3.0);

    let mut scene = Scene::new();
    scene.add(Box::new(prefabs::Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Vec3::from_scalar(1.0)
    )));
    scene.add(Box::new(prefabs::Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Vec3::from_scalar(1.0)
    )));

    Renderer::render(&scene, &mut camera);
    
    let screen = camera.take_screen().unwrap();

    copy_buffer(&mut output_image, &screen);

    output_image.save_as("output.png").unwrap();
}
