use super::Entity;

/// Abstraction of scene.
pub struct Scene {
    pub entities: Vec<Entity>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { entities: Vec::new() }
    }

    /// Add a new entity to scene.
    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}