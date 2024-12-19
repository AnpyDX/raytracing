use super::super::super::core::{
    Material, MatInput, Vec3
};

pub struct BgPure {
    pub color: Vec3
}

impl BgPure {
    pub fn new(color: Vec3) -> BgPure {
        BgPure { color }
    }
}

impl Material for BgPure {
    fn emissive(&self, _input: MatInput) -> Vec3 {
        self.color
    }
}