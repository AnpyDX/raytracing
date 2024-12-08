use super::{
    Camera,
    core::{
        Scene, Vec3,
        backends::{ 
            NativeRenderer, RenderTask, 
            STDrivenRenderer, STDrivenRendererConfig 
        }
    }
};

pub enum BackendConfig {
    /// Renderer running on single thread.
    CPUDrivenS,
    /// Renderer running on multiple thread.
    CPUDrivenM(u32),
}

pub enum DebugLevel {
    /// Running renderer with fully debug info.
    /// - Renderer configure.
    /// - Rendering progress.
    Full, 
    /// Running renderer with printing progress number only.
    Porting,
    /// Running renderer without any debug info.
    None
}

/// Renderer configurations
pub struct RendererConfig {
    /// Renderer backend.
    pub backend: BackendConfig,
    /// Debug level.
    pub dbg_level: DebugLevel,
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
        let backend_info = self.init_backend(screen);

        if let DebugLevel::Full = self.config.dbg_level {
            println!("Debug informations:");
            println!("backend: {}", backend_info);
            println!("entity number: {}", scene.entities.len());
            println!("resolution: {}x{}", screen.0, screen.1);
            println!("renderer: {} spp, {} depth", self.config.spp, self.config.max_depth);
        }

        let mut index = 0;
        let fb_size = (screen.0 * screen.1) as f64;
        let renderer = self.backend.as_mut().unwrap();

        for rays in camera.rays(screen, self.config.spp) {
            match self.config.dbg_level {
                DebugLevel::Full => {
                    let progress = index as f64 / fb_size;
                    Self::show_progress(progress);
                },
                DebugLevel::Porting => {
                    let progress = index as f64 / fb_size;
                    println!("{}", progress);
                },
                DebugLevel::None => {}
            }

            let task = RenderTask { rays, scene, index };
            renderer.submit(task);

            index += 1;
        }

        return renderer.fetch();
    }

    fn init_backend(&mut self, screen: (u32, u32)) -> String {
        let backend_info;

        match self.config.backend {
            BackendConfig::CPUDrivenS => {
                let renderer = STDrivenRenderer::new(
                    STDrivenRendererConfig {
                        fb_size: (screen.0 * screen.1) as usize,
                        max_depth: self.config.max_depth
                    }
                );

                self.backend = Some(Box::new(renderer));
                backend_info = "CPU driven, single-thread".to_string();
            },

            BackendConfig::CPUDrivenM(_) => { panic!("muti-thread renderer hasn't been impl yet!") }
        }

        return backend_info;
    }

    fn show_progress(percentage: f64) {
        let num_area = format!("{:.0}%", 100.0 * percentage);
        let prog_area = format!(
                "[{:=>50}]", 
                String::from(" ").repeat((50.0 * (1.0 - percentage)) as usize)
            );

        println!("{:>4} {}", num_area, prog_area);
    }
}