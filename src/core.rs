pub mod prefabs;

pub mod math;
pub use math::{ Vec2, Vec3, PI, INF };

mod ray;
use ray::Ray;

pub mod hittable;
use hittable::{ Hittable, SurfaceInfo };

pub mod scene;
pub use scene::Scene;

pub mod screen;
pub use screen::Screen;

pub mod camera;
pub use camera::Camera;

pub mod renderer;
pub use renderer::Renderer;