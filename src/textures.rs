use crate::vec3::*;
use image::RgbImage;

#[derive(Clone, Debug)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Vec<[u8; 3]>>
}

impl Picture {
    pub fn new(img: RgbImage) -> Self {
        let (width, height) = img.dimensions();
        let raw = img.into_vec();
        let mut pixels: Vec<Vec<[u8; 3]>> = vec![];
        let width3 = (width*3) as usize;
        for begin in (0..raw.len()).step_by(width3){
            pixels.push(vec![]);
            for pix in (begin..begin+width3).step_by(3){
                let last = pixels.len()-1;
                pixels[last].push([raw[pix], raw[pix+1], raw[pix+2]]);

            }
        }
        Self { width, height, pixels}
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vec3{
        let pix =  self.pixels[y][x];
        Vec3::new(pix[0] as f32/255., pix[1] as f32/255., pix[2] as f32/255.,)
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