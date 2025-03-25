use crate::{Vec3, render::Picture};

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
