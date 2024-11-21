pub use image::ImageError;
use image;

pub struct Image {
    pub buffer: Vec<u8>,
    pub size: (u32, u32)
}

impl Image {
    /// create a new PNG image object.
    /// 
    /// **NOTE:** all pixels will be initialized into (0, 0, 0)
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            buffer: vec![0; 3 * (width * height) as usize], 
            size: (width, height)
        }
    }

    /// write color into specfic pixel.
    /// 
    /// `position`: pixel's position `(x, y)`, 
    /// whose range is `0` to `width - 1 or height - 1`.
    /// 
    /// `color`: the color will be written into pixel, 
    /// whose format is `(r, g, b)` and range is `0 - 255`.
    pub fn write(&mut self, position: (u32, u32), color: (u8, u8, u8)) {
        let index = (self.size.0 * position.1 + position.0) * 3;
        let index = index as usize;

        self.buffer[index] = color.0;
        self.buffer[index + 1] = color.1;
        self.buffer[index + 2] = color.2;
    }

    /// save image as a PNG file.
    pub fn save_as(&self, name: &str) -> Result<(), ImageError>{
        image::save_buffer_with_format(
            name, &self.buffer, 
            self.size.0, self.size.1,
            image::ColorType::Rgb8,
            image::ImageFormat::Png
        )
    }
}