///! Core preludes.
pub mod core;

pub use core::math;

pub use core::{
    Camera, 
    Scene, Entity,
    Renderer, RendererConfig, BackendConfig, DebugLevel
};

pub use core::{
    Hittable,
    Material, MatInput, ShadeOutput,
    Lambertian, Emissive
};

///! Prefabs hittables.
pub mod prefabs;