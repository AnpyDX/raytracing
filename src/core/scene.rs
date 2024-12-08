use std::rc::Rc;
use super::{ Entity, Material };

/// Abstraction of scene.
pub struct Scene {
    pub entities: Vec<Entity>,
    pub background: Rc<dyn Material>
}

impl Scene {
    pub fn new(background: Rc<dyn Material>) -> Scene {
        Scene { entities: Vec::new(), background }
    }

    /// Add a new entity to scene.
    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}