use std::usize;
use image::{RgbImage, ImageBuffer, Rgb};
use crate::vec3;
use vec3::Vec3;
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

    pub fn as_vector(pix: &Self) -> Vec3{
        Vec3::new(pix.r as f32, pix.g as f32, pix.b as f32)
    }

    pub fn lerp(&self, other: &Self, value: f32) -> Self {
        let v = Self::as_vector(self).lerp(&Self::as_vector(other), value);
        Self::new(v.x as u8, v.y as u8, v.z as u8)
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