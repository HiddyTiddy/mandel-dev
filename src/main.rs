#![warn(clippy::unwrap_used, clippy::expect_used)]
// use image::Rgb;
use nalgebra::Complex;
mod gui;
// mod image;

const HEIGHT: usize = 1024;
const WIDTH: usize = 1024;

#[derive(Debug, Clone, Copy)]
pub struct Color(u8, u8, u8);
impl Color {
    pub fn to_rgba(self) -> [u8; 4] {
        [self.0, self.1, self.2, 255]
    }
}
// type Matrix = nalgebra::DMatrix<Complex<f64>>;

trait Imager {
    type Coordinate;
    fn color_at(&self, coordinate: Self::Coordinate) -> Color;
    fn new_with_size_hint(_width: usize, _height: usize) -> Self;
}

#[inline]
const fn color_map(iteration: usize) -> Color {
    match iteration {
        i @ 0..=254 => Color(255 - i as u8, 255 - i as u8, 255 - (i / 2) as u8),
        _ => Color(4, 0, 0),
    }
}

struct Mandelbrot;
impl Imager for Mandelbrot {
    type Coordinate = Complex<f64>;

    #[inline]
    fn color_at(&self, coordinate: Self::Coordinate) -> Color {
        let mut z = Complex::new(0.0, 0.0);
        let first = (0..75).find(|_| {
            z = z.powu(2) + coordinate;
            z.norm_sqr() > 4.0
        });

        match first {
            None => Color(0, 0, 0),
            Some(it) => color_map(it),
        }
    }

    fn new_with_size_hint(_width: usize, _height: usize) -> Self {
        Mandelbrot
    }
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;

    gui::gui();

    Ok(())
}
