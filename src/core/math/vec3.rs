use core::f64;
use std::{
    ops::{ Neg, Add, Sub, Mul, Div },
    cmp::PartialEq
};
use super::Interval;
use rand::{ self, Rng };

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64, pub y: f64, pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Create vec3 from scalar.
    pub fn from_scalar(scalar: f64) -> Vec3 {
        Vec3 { x: scalar, y: scalar, z: scalar }
    }

    /// Generate a random vec3 with each element containing in given interval.
    pub fn random(interval: Interval) -> Vec3 {
        let range = interval.min..=interval.max;
        let mut rng = rand::thread_rng();
        Vec3 { 
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range)
        }
    }

    /// Generate a random unit vec3.
    pub fn random_unit() -> Vec3 {
        loop {
            let v = Self::random(Interval::new(-1.0, 1.0));
            if Interval::new(1e-160, 1.0).contains(v.length_square()) {
                return v.normalized();
            }
        }
    }
    
    /// Get length's square of vec3 itself.
    pub fn length_square(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Get length of vec3 itself.
    pub fn length(&self) -> f64 {
        self.length_square().sqrt()
    }

    /// Return a normalized vec3 of itself.
    pub fn normalized(&self) -> Self {
        let len = self.length();

        match len {
            0.0 => self.clone(),
            _ => self.clone() / len
        }
    }

    pub fn dot(&self, right: Vec3) -> f64 {
        self.x * right.x + self.y * right.y + self.z * right.z
    }

    pub fn cross(&self, right: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * right.z - self.z * right.y,
            y: self.z * right.x - self.x * right.z,
            z: self.x * right.y - self.y * right.x
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs
        }
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        -rhs + self
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z
        }
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::Vec3;

    #[test]
    fn neg() {
        assert_eq!(-Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn add_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::from_scalar(-1.0);

        assert_eq!(v1 + v2, Vec3::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn add_f64() {
        let v1 = Vec3::new(10.0, 5.0, 0.0);

        assert_eq!(v1 + 2.0, Vec3::new(12.0, 7.0, 2.0));
        assert_eq!(2.0 + v1, Vec3::new(12.0, 7.0, 2.0));
    }

    #[test]
    fn sub_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::from_scalar(-1.0);

        assert_eq!(v1 - v2, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn sub_f64() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        
        assert_eq!(v1 - 2.0, Vec3::new(-1.0, 0.0, 1.0));
        assert_eq!(2.0 - v1, Vec3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);

        assert_eq!(v1 * v2, Vec3::new(2.0, 2.0, 0.0));
    }

    #[test]
    fn mul_f64() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v1 * 3.0, Vec3::new(3.0, 6.0, 9.0));
        assert_eq!(3.0 * v1, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn div_vec3() {
        let v1 = Vec3::new(2.0, 4.0, 6.0);
        let v2 = Vec3::from_scalar(2.0);

        assert_eq!(v1 / v2, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn div_f64() {
        let v1 = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(v1 / 2.0, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(12.0 / v1, Vec3::new(6.0, 3.0, 2.0));
    }

    #[test]
    fn length() {
        let v1 = Vec3::from_scalar(0.0);
        let v2 = Vec3::new(0.0, 3.0, 4.0);

        assert_eq!(v1.length(), 0.0);
        assert_eq!(v2.length(), 5.0);
    }

    #[test]
    fn normalize() {
        let v1 = Vec3::from_scalar(0.0);
        let v2 = Vec3::new(0.0, 3.0, 4.0);

        assert_eq!(v1.normalized(), v1);
        assert_eq!(v2.normalized(), Vec3::new(0.0, 0.6, 0.8));
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(0.0, 1.0, 2.0);

        assert_eq!(v1.dot(v2), 8.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(0.0, 1.0, 2.0);

        // x = 2.0 * 2.0 - 3.0 * 1.0 = 1.0
        // y = 3.0 * 0.0 - 1.0 * 2.0 = -2.0
        // z = 1.0 * 1.0 - 2.0 * 0.0 = 1.0

        assert_eq!(v1.cross(v2), Vec3::new(1.0, -2.0, 1.0));
    }
}