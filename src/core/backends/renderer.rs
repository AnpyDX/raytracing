use std::sync::Arc;
use super::super::{ Ray, Entity, Vec3 };

/// Renderer's Render Task
pub struct RenderTask {
    /// The ray that is going to send into the scene.
    pub ray: Ray,
    /// Rendering scene's ref.
    pub scene: Arc<Vec<Entity>>,
    /// The background color.
    pub background: Vec3,
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