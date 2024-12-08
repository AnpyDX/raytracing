pub mod math;
pub use math::{ Vec2, Vec3, Interval };

mod ray;
pub use ray::Ray;

pub mod hittable;
pub use hittable::{ HittingInfo, Hittable };

pub mod material;
pub use material::{ 
    MatInput, ShadeOutput, Material,
    Lambertian, Emissive
};

pub mod entity;
pub use entity::Entity;

pub mod scene;
pub use scene::Scene;

pub mod backends;