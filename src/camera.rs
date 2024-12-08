use rand::{ self, Rng };
use super::core::{ Ray, Vec3, Vec2 };

/// Camera abstraction
#[derive(Clone, Copy)]
pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub fov: f64,
    pub viewport: Vec2, 
}

/// Pixel Rendering Task Iterator
pub struct RayCollection {
    /// The owner camera of iterator.
    owner: Camera,
    /// the left-upper point of viewport (aka the beginning point).
    origin: Vec3,
    /// viewport's UV coordinate, 
    /// 
    /// which is **flipped** and contains size info(not normalized).
    /// 
    /// **NOTE**
    /// - *U*'s direction is right.
    /// - *V*'s direction is down.
    /// - UV coordinate origin point is in left-upper corner.
    uvdir: (Vec3, Vec3),
    /// screen size.
    screen: (u32, u32),
    /// The number of sample time per pixel.
    spp: u32,

    // Helper variables for iteration.
    ending: usize,
    counter: usize
}

impl Camera {
    /// Create a new camera.
    /// 
    /// **NOTE:** all arguments follow left-head coord.
    pub fn new(position: Vec3, direction: Vec3, viewport: Vec2, fov: f64) -> Camera {
        Camera {
            pos: position, 
            dir: direction,
            fov,
            viewport,
        }
    }

    /// Get ray's collection.
    pub fn rays(&self, screen: (u32, u32), spp: u32) -> RayCollection {
        RayCollection::new(*self, screen, spp)
    }
}

impl RayCollection {
    pub fn new(owner: Camera, screen: (u32, u32), spp: u32) -> RayCollection {
        // Caculate the world position of 
        // the first pixel(on left-upper side) of viewport.
        let cam_right = Vec3::new(0.0, 1.0, 0.0)
                                    .cross(owner.dir)
                                    .normalized();
        let cam_up = owner.dir.cross(cam_right)
                                    .normalized();
        let left_v = -cam_right * owner.viewport.x / 2.0;
        let up_v = cam_up * owner.viewport.y / 2.0;

        let uvdir = (cam_right * owner.viewport.x, -cam_up * owner.viewport.y);

        let focal_len = owner.viewport.y / ( 2.0 * f64::tan(owner.fov));
        let origin = owner.pos + owner.dir.normalized() * focal_len + left_v + up_v;

        let ending = (screen.0 * screen.1) as usize;

        RayCollection { owner, origin, uvdir, screen, spp, ending, counter: 0 }
    }
}

impl Iterator for RayCollection {
    type Item = Vec<Ray>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == self.ending {
            return None;
        }
        else {
            let mut rays = Vec::new();

            for _ in 0..self.spp {
                let offset = Vec2::random(0.0, 1.0) - 0.5;

                let u = self.counter % self.screen.0 as usize;
                let v = self.counter / self.screen.0 as usize;
    
                let u_scaler = (u as f64 + offset.x) / self.screen.0 as f64;
                let v_scaler = (v as f64 + offset.y) / self.screen.1 as f64;
                let direction = self.origin + self.uvdir.0 * u_scaler + self.uvdir.1 * v_scaler;
    
                let ray = Ray::new(self.owner.pos, direction);

                rays.push(ray);
            }

            self.counter += 1;

            return Some(rays);
        }
    }
}