use std::rc::Rc;
use super::{ Material, Hittable };

pub struct Entity {
    /// Material of enity.
    pub mat: Rc<dyn Material>,
    /// Mesh of enity.
    pub mesh: Rc<dyn Hittable>
}

impl Entity {
    /// Create a new enity with a material and mesh.
    pub fn new(mat: Rc<dyn Material>, mesh: Rc<dyn Hittable>) -> Entity {
        Entity { mat, mesh }
    }
}