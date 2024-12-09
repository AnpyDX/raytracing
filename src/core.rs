pub mod math;
pub use math::{ Vec2, Vec3, Interval };

mod ray;
pub use ray::Ray;

mod backends;

pub mod hittable;
pub use hittable::{ Hittable, HittingInfo };

pub mod material;
pub use material::{ 
    Material, MatInput, ShadeOutput,
    Lambertian, Emissive
};

pub mod entity;
pub use entity::Entity;

pub mod scene;
pub use scene::Scene;

pub mod camera;
pub use camera::Camera;

pub mod renderer;
pub use renderer::{ Renderer, RendererConfig, BackendConfig, DebugLevel };