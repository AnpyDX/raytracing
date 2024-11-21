use std::ops::{ Neg, Add, Sub, Mul, Div };
use std::cmp::PartialEq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64, pub y: f64
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    /// Create vec2 from scalar.
    pub fn from_scalar(scalar: f64) -> Vec2 {
        Vec2 { x: scalar, y: scalar }
    }

    /// Get the length of vec2 itself.
    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Return a normalized vec3 of itself
    pub fn normalized(&self) -> Vec2 {
        let len = self.length();

        match len {
            0.0 => self.clone(),
            _ => self.clone() / len
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2 { x: -self.x, y: -self.y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<f64> for Vec2 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Add<Vec2> for f64 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        rhs + self
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<f64> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<Vec2> for f64 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        -rhs + self
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Div<Vec2> for f64 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self / rhs.x,
            y: self / rhs.y
        }
    }
}

#[cfg(test)]
mod vec2_tests {
    use super::Vec2;

    #[test]
    fn neg() {
        assert_eq!(-Vec2::from_scalar(0.0), Vec2::from_scalar(0.0));
        assert_eq!(-Vec2::new(1.0, 2.0), Vec2::new(-1.0, -2.0));
    }

    #[test]
    fn add_vec2() {
        let v1 = Vec2::new(2.0, 2.0);
        let v2 = Vec2::new(-1.0, 1.0);

        assert_eq!(v1 + v2, Vec2::new(1.0, 3.0));
    }

    #[test]
    fn add_f64() {
        let v1 = Vec2::new(1.0, 2.0);

        assert_eq!(v1 + 3.0, Vec2::new(4.0, 5.0));
        assert_eq!(3.0 + v1, Vec2::new(4.0, 5.0));
    }

    #[test]
    fn sub_vec2() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(5.0, 6.0);

        assert_eq!(v1 - v2, Vec2::new(-4.0, -4.0));
    }

    #[test]
    fn sub_f64() {
        let v1 = Vec2::new(1.0, 2.0);

        assert_eq!(v1 - 5.0, Vec2::new(-4.0, -3.0));
        assert_eq!(5.0 - v1, Vec2::new(4.0, 3.0));
    }

    #[test]
    fn mul_vec2() {
        let v1 = Vec2::new(10.0, 20.0);
        let v2 = Vec2::new(0.2, 0.1);

        assert_eq!(v1 * v2, Vec2::from_scalar(2.0));
    }

    #[test]
    fn mul_f64() {
        let v1 = Vec2::new(5.0, 6.0);

        assert_eq!(v1 * 2.0, Vec2::new(10.0, 12.0));
        assert_eq!(2.0 * v1, Vec2::new(10.0, 12.0));
    }

    #[test]
    fn div_vec2() {
        let v1 = Vec2::new(10.0, 12.0);
        let v2 = Vec2::new(2.0, 6.0);

        assert_eq!(v1 / v2, Vec2::new(5.0, 2.0));
    }

    #[test]
    fn div_f64() {
        let v1 = Vec2::new(10.0, 12.0);
        let v2 = Vec2::new(2.0, 3.0);

        assert_eq!(v1 / 2.0, Vec2::new(5.0, 6.0));
        assert_eq!(12.0 / v2, Vec2::new(6.0, 4.0));
    }

    #[test]
    fn length() {
        assert_eq!(Vec2::from_scalar(0.0).length(), 0.0);
        assert_eq!(Vec2::new(3.0, 4.0).length(), 5.0);
    }

    #[test]
    fn normalize() {
        let v1 = Vec2::from_scalar(0.0);
        let v2 = Vec2::new(3.0, 4.0);

        assert_eq!(v1.normalized(), v1);
        assert_eq!(v2.normalized(), Vec2::new(0.6, 0.8));
    }
}