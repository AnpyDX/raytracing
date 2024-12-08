use std::rc::Rc;
use super::super::{ Scene, Ray, Vec3 };

/// Renderer's Render Task
pub struct RenderTask<'a> {
    /// The ray set, in which rays' tracing results are sampled into the same pixel.
    pub rays: Vec<Ray>,
    /// Rendering scene's ref.
    pub scene: &'a Scene,
    /// The buffer's index where the rendering result will be put.
    pub index: usize
}

/// Abstraction for Native Renderer
pub trait NativeRenderer {
    /// Submit a RenderTask to renderer.
    fn submit(&mut self, task: RenderTask);

    /// Fetch the buffer from renderer.
    fn fetch(&self) -> Vec<Vec3>;
}