use super::{ Material, Hittable };

pub struct Entity {
    /// Material of enity.
    pub mat: Box<dyn Material>,
    /// Mesh of enity.
    pub mesh: Box<dyn Hittable>
}

impl Entity {
    /// Create a new enity with a material and mesh.
    pub fn new(mat: Box<dyn Material>, mesh: Box<dyn Hittable>) -> Entity {
        Entity { mat, mesh }
    }
}