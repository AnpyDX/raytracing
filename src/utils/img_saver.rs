//! Save image file from pixels buffer.
use crate::math::{ Vec3, Interval };
use std::ptr::slice_from_raw_parts;
use image::{ self, ColorType };

/// Save buffer as image.
pub fn save_as(
    name: &str, 
    buffer: &Vec<Vec3>, 
    color_t: ColorType, 
    width: u32, 
    height: u32
) -> Result<(), String> {
    // Do Gamma correction.
    let linear_to_gamma = |comp: f64| -> f64 {
        if comp <= 0.0 { return 0.0 }
        return comp.sqrt();
    };

    let pixels = buffer.iter().map(|v| {
        let x = linear_to_gamma(v.x);
        let y = linear_to_gamma(v.y);
        let z = linear_to_gamma(v.z);
        Vec3 { x, y, z }
    });

    // Transfer pixels into Vec<u8>.
    let intensity = Interval::new(0.0, 0.999);
    match color_t {
        ColorType::Rgb8 => {
            let mut buf = Vec::with_capacity(pixels.len() * 3);

            for p in pixels {
                buf.push((u8::MAX as f64 * intensity.clamp(p.x)) as u8);
                buf.push((u8::MAX as f64 * intensity.clamp(p.y)) as u8);
                buf.push((u8::MAX as f64 * intensity.clamp(p.z)) as u8);
            }

            if let Err(err) = image::save_buffer(name, &buf, width, height, color_t) {
                return Err(format!("{:?}", err));
            }
        },
        ColorType::Rgb16 => {
            let mut buf = Vec::with_capacity(pixels.len() * 3);

            for p in pixels {
                buf.push((u16::MAX as f64 * intensity.clamp(p.x)) as u16);
                buf.push((u16::MAX as f64 * intensity.clamp(p.y)) as u16);
                buf.push((u16::MAX as f64 * intensity.clamp(p.z)) as u16);
            }

            let buf_ref = unsafe { 
                let ptr = buf.as_ptr() as *const u16 as *const u8;
                slice_from_raw_parts(ptr, buf.len() * 2).as_ref().unwrap()
            };

            if let Err(err) = image::save_buffer(name, buf_ref, width, height, color_t) {
                return Err(format!("{:?}", err));
            }
        },
        _ => {
            return Err(format!("color_t {:?} is not supported", color_t));
        }
    }

    return Ok(());
}