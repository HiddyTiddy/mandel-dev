//! unused

use std::{fs::File, io::BufWriter, path::Path};

/// A RGB image
pub struct Image {
    width: usize,
    height: usize,
    // buffer is guaranteed to have lenght width * height * 3
    buffer: Box<[u8]>,
}



impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[allow(unused)]
    pub const BLACK: Self = Rgb::new(0, 0, 0);
    #[allow(unused)]
    pub const WHITE: Self = Rgb::new(255, 255, 255);
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height * 3]; // R, G, B
        let buffer = Box::from(buffer);

        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn from_buffer(width: usize, height: usize, buffer: Vec<u8>) -> Self {
        let buffer = Box::from(buffer);
        Self {
            width,
            height,
            buffer,
        }
    }

    #[inline]
    fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width * 3 + x * 3
    }

    #[allow(unused)]
    pub fn get_pixel(&self, x: usize, y: usize) -> Rgb {
        let index = self.to_index(x, y);
        Rgb::new(
            self.buffer[index],
            self.buffer[index + 1],
            self.buffer[index + 2],
        )
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Rgb) {
        let index = self.to_index(x, y);
        self.buffer[index] = pixel.r;
        self.buffer[index + 1] = pixel.g;
        self.buffer[index + 2] = pixel.b;
    }

    pub fn write(&self, path: &str) -> color_eyre::eyre::Result<()> {
        let path = Path::new(path);
        let file = File::create(path)?;
        let mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(&mut w, self.width as _, self.height as _);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        writer.write_image_data(&self.buffer)?;

        Ok(())
    }
}
