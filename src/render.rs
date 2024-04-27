use image::{Rgb, RgbImage, ImageBuffer};
use crate::{WIDTH, HEIGHT};
#[derive(Clone, Copy, Default)]
pub struct Pixel{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self{r, g, b}
    }
}

pub fn display(func: impl Fn(usize, usize)-> Pixel, name: &str) {
    let mut buffer: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let col = func(x as usize, y as usize);
        *pixel = Rgb([col.r, col.g, col.b]);
    }
    buffer.save(format!("{}.png", name)).unwrap();
}