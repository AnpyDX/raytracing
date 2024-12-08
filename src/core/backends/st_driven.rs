use std::{ rc::Rc, f64::INFINITY };
use super::super::{ 
    Ray, Vec3, Vec2, Interval,
    Scene, Material, MatInput, HittingInfo
};
use super::renderer::{ NativeRenderer, RenderTask };

pub struct STDrivenRendererConfig {
    /// The size of framebuffer, which generally is the number of pixel.
    pub fb_size: usize,
    /// The maximum number of ray bounce depth.
    pub max_depth: u32
}

/// Single-thread CPU Renderer.
pub struct STDrivenRenderer {
    framebuffer: Vec<Vec3>,
    config: STDrivenRendererConfig
}

impl STDrivenRenderer {
    pub fn new(config: STDrivenRendererConfig) -> STDrivenRenderer {
        STDrivenRenderer {
            framebuffer: vec![Vec3::from_scalar(0.0); config.fb_size],
            config
        }
    }

    fn hit_scene(ray: &Ray, scene: &Scene, step_limit: Interval) -> Option<(HittingInfo, Rc<dyn Material>)> {
        let mut min_step = INFINITY;
        let mut current_hit = None;
        let mut current_mat = None;

        for i in 0..scene.entities.len() {
            if let Some(hit) = scene.entities[i].mesh.hit(ray, step_limit) {
                if min_step > hit.step {
                    min_step = hit.step;
                    current_hit = Some(hit);
                    current_mat = Some(Rc::clone(&scene.entities[i].mat));
                }
            }
        }

        if let Some(hit_info) = current_hit {
            return Some((hit_info, current_mat.unwrap()));
        }
        else { return None; }
    }

    fn ray_color(ray: Ray, depth: u32, scene: &Scene) -> Vec3 {
        /*
         * There are three situations that `ray_color` will return:
         * 1. if ray hit a light(emissive material), return light color.
         * 
         * 2. if ray's step exceeds the limit, it means the ray will 
         *    deeper into the background. So just return bg_color.
         * 
         * 3. if `depth <= 0`, it means the ray never reach a light source,
         *    just return (0.0, 0.0, 0.0) to make pixel's color dark.
        */

        if depth <= 0 { return Vec3::from_scalar(0.0); }

        let step_limit = Interval::new(0.001, INFINITY);

        let Some((rec, mat)) = Self::hit_scene(&ray, scene, step_limit)
        else {
            let bg_input = MatInput {
                incident_ray: ray,
                // the next three arguments are uesless.
                surface_norm: Vec3::from_scalar(0.0),
                surface_front: true,
                hitted_position: Vec3::from_scalar(0.0)
            };
            return scene.background.emissive(bg_input);
        };

        let mat_input = MatInput {
            incident_ray: ray,
            surface_norm: rec.normal,
            surface_front: rec.is_front,
            hitted_position: rec.position
        };
        
        let emissive_color = mat.emissive(mat_input);

        let Some(shade_output) = mat.shade(mat_input) else {
            return emissive_color;
        };

        let scatter_color = shade_output.attenuation * Self::ray_color(shade_output.scatter, depth - 1, scene);

        return emissive_color + scatter_color;
    }
}

impl NativeRenderer for STDrivenRenderer {
    fn submit(&mut self, task: RenderTask) {
        if task.rays.is_empty() { return; }

        let sample_scalar = 1.0 / task.rays.len() as f64;
        
        for ray in task.rays {
            self.framebuffer[task.index] += sample_scalar * Self::ray_color(ray, self.config.max_depth, task.scene);
        }
    }

    fn fetch(&self) -> Vec<Vec3> {
        self.framebuffer.clone()
    }
}