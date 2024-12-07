use rand::prelude::*;
use super::{
    Scene, Camera, Ray, Hittable, 
    math::{ Vec3, Interval, INF }
};

pub enum BackendConfig {
    /// Renderer running on single thread.
    CPUDrivenS,
    /// Renderer running on multiple thread.
    CPUDrivenM(u32),
    /// Renderer running on GPU (Vulkan).
    GPUDriven
}

pub enum DebugLevel {
    /// Running renderer with fully debug info.
    /// - Renderer configure.
    /// - Rendering progress.
    Full, 
    /// Running renderer with config info only.
    /// - Renderer configure.
    Brief,
    /// Running renderer without any debug info.
    None
}

/// Renderer configurations.
pub struct RendererConfig {
    /*
     * Fixed Configurations
     * cannot be changed after creation.
    */

    /// Renderer backend.
    backend: BackendConfig,
    /// Debug level.
    dbg_level: DebugLevel,

    /*
     * Flexible Configurations
     * can be change after creation.
    */
    /// Sample per pixel.
    spp: u32
}

impl Default for RendererConfig {
    fn default() -> Self {
        RendererConfig {
            backend: BackendConfig::CPUDrivenS,
            dbg_level: DebugLevel::Full,
            spp: 32
        }
    }
}

// TODO do abstraction for backends-renderer.
//      what main renderer needs to do is to 
//      assign tasks.
pub struct Renderer {
    config: RendererConfig,
}

impl Renderer {
    pub fn new(config: RendererConfig) -> Renderer {
        Renderer { config }
    }

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