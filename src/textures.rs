use crate::vec3::Vec3;
use image::RgbImage;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub data: Arc<Vec<u8>>,
}

impl Picture {
    pub fn new(img: RgbImage) -> Self {
        let (width, height) = img.dimensions();
        Self { width, height, data: Arc::new(img.into_vec())}
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vec3{
        let y_index = y*(self.width*3) as usize;
        let index = x*3+y_index;
        Vec3::new(self.data[index] as f32/255.0,
                self.data[index+1] as f32/255.0,
                self.data[index+2] as f32/255.0)
    }
}

#[derive(Clone, Debug)]
pub enum Texture {
    Solid {color: Vec3},
    Img {img: Picture},
    Checker {color1: Vec3, color2: Vec3, size: f32}
}

impl Texture{
    pub fn sphere_uv_coord(center: &Vec3, hitp: &Vec3) -> (f32, f32) {
        let heading: Vec3 = (hitp - center).normalize();
        let theta = heading.z.acos();
        let phi = heading.x.atan2(-heading.y);
        let pi = std::f32::consts::PI;
        let v = theta / pi;
        let u =(phi + pi) / (2. * pi);
        (u, v)
    }
}
