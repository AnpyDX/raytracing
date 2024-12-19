use super::super::super::core::{
    Material, MatInput, ShadeOutput, Vec3
};

pub struct Emissive {
    pub emissive: Vec3
}

impl Emissive {
    pub fn new(emissive: Vec3) -> Emissive {
        Emissive { emissive }
    }
}

impl Material for Emissive {
    fn emissive(&self, _input: MatInput) -> Vec3 {
        self.emissive
    }

    fn shade(&self, _input: MatInput) -> Option<ShadeOutput> {
        None
    }
}