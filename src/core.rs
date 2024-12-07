pub mod math;
pub use math::{ Vec2, Vec3, Interval };

mod ray;
use ray::Ray;

pub mod hittable;
use hittable::{ HittingInfo, Hittable };

pub mod material;
pub use material::{ MatInput, ShadeOutput, Material };

pub mod entity;
pub use entity::Entity;

pub mod scene;


pub mod backends;
pub use backends::renderer::{ RenderTask, NativeRenderer };
pub use backends::st_driven::STDrivenRenderer;