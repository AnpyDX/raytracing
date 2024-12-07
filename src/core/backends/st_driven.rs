use std::f64::INFINITY;
use super::super::{ 
    Ray, Vec3, Interval,
    Entity, Material, MatInput, HittingInfo
};
use super::renderer::{ NativeRenderer, RenderTask };

pub struct STDrivenRendererConfig {
    /// The size of framebuffer, which generally is the number of pixel.
    pub fb_size: usize,
    /// The maximum number of ray bounce depth.
    pub max_depth: u32,
    /// The number of sample-times per pixel.
    pub spp: u32
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

    fn hit_scene(ray: &Ray, scene: &Vec<Entity>, step_limit: Interval) -> Option<(HittingInfo, Box<dyn Material>)> {
        let min_step = INFINITY;
        let has_hitted = false;
        let current_mat;
        let current_hit;

        for i in 0..scene.len() {
            if let Some(hit) = scene[i].mesh.hit(ray, step_limit) {
                has_hitted = true;
                if min_step > hit.step {
                    min_step = hit.step;
                    current_mat = Box::clone(scene[i].mat);
                    current_hit = hit;
                }
            }
        }

        if !has_hitted { return None; }

        Some((current_hit, current_mat))
    }

    fn ray_color(ray: Ray, depth: u32, scene: &Vec<Entity>, bg: Vec3) -> Vec3 {
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
        else { return bg; };

        let mat_input = MatInput {
            surface_norm: rec.normal,
            surface_front: rec.is_front,
            hitted_position: rec.position
        };

        
        let emissive_color = mat.emissive(mat_input);

        let scatter_color;
        if let Some(shade_output) = mat.shade(mat_input) {
            scatter_color = shade_output.attenuation * Self::ray_color(shade_output.scatter, depth - 1, scene, bg);
        }
        else {
            return emissive_color;
        }

        return emissive_color + scatter_color;
    }
}

impl NativeRenderer for STDrivenRenderer {
    fn submit(&mut self, task: RenderTask) {
        
    }

    fn fetch(&self) -> Vec<Vec3> {
        self.framebuffer.clone()
    }
}