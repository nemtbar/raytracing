use std::usize;
use image::{RgbImage, ImageBuffer, Rgb};
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

pub fn display(func: impl Fn(usize, usize)-> Pixel) {
    let mut buffer: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let col = func(x as usize, y as usize);
        *pixel = Rgb([col.r, col.g, col.b]);
    }
    buffer.save("sample.png").unwrap();
}

pub fn transform(r: f32, g: f32, b: f32) -> Pixel {
    Pixel::new((r * 255.) as u8, (g* 255.) as u8, (b* 255.) as u8)
}