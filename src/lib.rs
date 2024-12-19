///! Core preludes.
pub mod core;

pub use core::math;

pub use core::{
    Camera, 
    Scene, Entity,
    Renderer, RendererConfig, BackendConfig
};

pub use core::{
    Hittable,
    Material, MatInput, ShadeOutput
};

///! Prefabs shpaes and materials.
pub mod prefabs;

///! Utilities.
pub mod utils;