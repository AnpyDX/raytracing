///! Collection of useful constants and functions.
use core::f64;

/// Convert degress to radians.
pub fn radians(degress: f64) -> f64 {
    f64::consts::PI * degress / 180.0
}