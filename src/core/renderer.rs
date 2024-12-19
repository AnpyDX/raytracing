use super::{ Camera, Scene, Vec3 };
use super::backends::{ 
    NativeRenderer, RenderTask, 
    STDrivenRenderer, STDrivenRendererConfig 
};

#[derive(Debug)]
pub enum BackendConfig {
    /// Renderer running on single thread.
    CPUDrivenS,
    /// Renderer running on multiple thread.
    CPUDrivenM(u32),
}

/// Renderer configurations
pub struct RendererConfig {
    /// Renderer backend.
    pub backend: BackendConfig,
    /// The maximum number of ray bounce depth.
    pub max_depth: u32,
    /// The number of sample-times per pixel.
    pub spp: u32
}

pub struct Renderer {
    backend: Option<Box<dyn NativeRenderer>>,
    config: RendererConfig
}

impl Renderer {
    pub fn new(config: RendererConfig) -> Renderer {
        Renderer { backend: None, config }
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera, screen: (u32, u32)) -> Vec<Vec3> {
        self.init_backend(screen);

        let mut index = 0;
        let fb_size = (screen.0 * screen.1) as f64;
        let renderer = self.backend.as_mut().unwrap();

        for rays in camera.rays(screen, self.config.spp) {
            let progress = index as f64 / fb_size;
            Self::show_progress(progress);

            let task = RenderTask { rays, scene, index };
            renderer.submit(task);

            index += 1;
        }

        return renderer.fetch();
    }

    fn init_backend(&mut self, screen: (u32, u32)) {
        match self.config.backend {
            BackendConfig::CPUDrivenS => {
                let renderer = STDrivenRenderer::new(
                    STDrivenRendererConfig {
                        fb_size: (screen.0 * screen.1) as usize,
                        max_depth: self.config.max_depth
                    }
                );

                self.backend = Some(Box::new(renderer));
            },

            BackendConfig::CPUDrivenM(_) => { panic!("muti-thread renderer hasn't been impl yet!") }
        }
    }

    fn show_progress(percentage: f64) {
        let num_area = format!("{:.0}%", 100.0 * percentage);
        let prog_area = format!(
                "[{:<50}]", 
                String::from("=").repeat((50.0 * percentage) as usize) + ">"
            );

        print!("{:>4} {}\r", num_area, prog_area);
    }
}