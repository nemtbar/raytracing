mod geometry;
mod render;
mod vec3;
mod textures;
use image::RgbImage;
#[allow(unused_imports)]
use image::{ImageBuffer, ImageReader};
use geometry::{Camera, Material, Object, QuadType, Reflection};
use rand::Rng;
use render::{display, transform, Pixel};
use textures::{Texture, Picture};
use std::env;
use vec3::Vec3;
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html

pub struct Uniforms {
    pub sample_count: u32,
    pub bounce_count: u8,
    pub offset: f32,
    pub cam: Camera,
    pub objects: Vec<Object>,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self { sample_count: 100, bounce_count: 50, offset: WIDTH as f32/1000., cam: Camera::default(), objects: vec![] }
    }
    
}

impl Uniforms {
    pub fn new(sample_count: u32, bounce_count: u8, offset: f32, cam: Camera, objects: Vec<Object>) -> Self {
        Self {sample_count, bounce_count, offset, cam, objects}
    }
}
//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize, input: &Uniforms) -> Pixel {
    let mut rng = rand::thread_rng();
    let mut color_sum = Vec3::default();
    for _ in 0..input.sample_count {
        let rand_x = rng.gen_range(-input.offset..input.offset);
        let rand_y = rng.gen_range(-input.offset..input.offset);
        let ray = input.cam.shoot(x as f32 + rand_x, y as f32 + rand_y);
        color_sum = color_sum + Object::bounce(&ray, &input.objects, input.bounce_count);
    }
    //average of color samples
    color_sum = color_sum / input.sample_count as f32;
    transform(color_sum.x, color_sum.y, color_sum.z)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let image: RgbImage = ImageReader::open("earthmap.jpg").unwrap().decode().unwrap().into_rgb8();
    let img = Picture::new(image);
    let up = Vec3::new(0., 0., 1.);
    let back = Vec3::new(0., 1., 0.);
    let side = Vec3::new(1., 0., 0.);
    let mirror = Material{refl:Reflection::Metal { roughness: 0. }, tex: Texture::Solid { color: Vec3::new1(1.) }};
    let input = Uniforms {
        sample_count: 200, 
        bounce_count: 50,
        offset: WIDTH as f32/1000.,
        cam: Camera::new(
            &Vec3::new(0., 0., 3.),
            &Vec3::new(-3., 3., 3.),
            95., 
            &Vec3::new(0., 0., 1.),
            0.
        ),
        objects: vec![
            Object::Sphere { pos: Vec3::new(-1., 3., 1.5), rad: 1.5, mat: Material{refl: Reflection::Diffuse(), tex: Texture::Img { img }} },
            Object::Quad { pos: Vec3::new(-3., 0., 0.), delta_x: &side*6., delta_y: &back*6., kind: QuadType::Rect(), mat: Material::default() },
            Object::Quad { pos: Vec3::new(-3., 6., 0.), delta_x: &side*6., delta_y: &up*6., kind: QuadType::Rect(), mat: Material::default() },
            Object::Quad { pos: Vec3::new(-3., 0., 0.), delta_x: &back*6., delta_y: &up*6., kind: QuadType::Rect(), mat: mirror.clone() },
            Object::Quad { pos: Vec3::new(3., 0., 0.), delta_x: &back*6., delta_y: &up*6., kind: QuadType::Rect(), mat: mirror },
            //Object::Quad { pos: Vec3::new(-3., 0., 6.), delta_x: &side*6., delta_y: &back*6., kind: QuadType::Rect(), mat: Material::default() }
        ]
    };

    display(frag, input, "sample");
}
