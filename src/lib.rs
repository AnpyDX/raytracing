pub mod utils;

pub mod core;
pub use core::{
    Scene, Entity, Hittable,
    Material, MatInput, ShadeOutput, 
    Lambertian, Emissive,
    Vec3, Vec2, math::utils::radians
};

pub mod camera;
pub use camera::Camera;

pub mod prefabs;
pub use prefabs::Sphere;

pub mod renderer;
pub use renderer::{ BackendConfig, DebugLevel, RendererConfig, Renderer };