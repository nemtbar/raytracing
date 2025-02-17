use crate::vec3::*;
use image::RgbImage;

#[derive(Clone, Debug)]
pub enum Texture {
    Solid {color: Vec3},
    Img {img: RgbImage},
    Checker {color1: Vec3, color2: Vec3, size: f32}
}

impl Texture{
    pub fn sphere_uv_coord(center: &Vec3, hitp: &Vec3) -> (f32, f32) {
        let heading: Vec3 = (hitp - center).normalize();
        let theta = heading.z.acos();
        let phi = heading.y.atan2(heading.x);
        let pi = std::f32::consts::PI;
        let v = theta / pi;
        let u =(phi + pi) / (2. * pi);
        (u, v)
    }
}