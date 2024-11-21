use super::math::Vec3;

/// abstraction of buffer, which is used
/// to store pixels buffer.
pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<Vec3>
}

impl Screen {
    /// create a new screen, with buffer memory allocated.
    pub fn new(width: u32, height: u32) -> Screen {
        Screen {
            width, height,
            buffer: vec![Vec3::from_scalar(0.0); (width * height) as usize]
        }
    } 

    /// write pixel color of screen.
    pub fn write(&mut self, x: u32, y: u32, color: Vec3) {
        let index = y * self.width + x;
        let index = index as usize;
        self.buffer[index] = color;
    }
}