mod geometry;
mod render;
mod vec3;
mod textures;
use image::RgbImage;
#[allow(unused_imports)]
use image::{ImageBuffer, ImageReader};
use geometry::{Camera, Material, Object, Planes, Reflection};
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
    let _img = Picture::new(image);
    let input = Uniforms {
        sample_count: 50, 
        bounce_count: 50,
        offset: WIDTH as f32/1000.,
        cam: Camera::new(
            &Vec3::new(0., -7., 6.),
            &Vec3::default(),
            95., 
            &Vec3::new(0., 0., 1.),
            0.
        ),
        objects: vec![
            Object::Sphere { pos: Vec3::new(-5.5, 0.5, 3.), rad: 2., mat: Material {refl: Reflection::Metal { roughness: 0. }, tex: Texture::Solid { color: Vec3::new1(1.) }} },
            Object::Plane { pos: Vec3::new(-2., 3., 3.), 
                normal: Vec3::new(0., -1., 0.), 
                mat: Material {refl: Reflection::Diffuse(), tex: Texture::Solid { color: Vec3::new(1., 0., 0.)}},
                plane_type: Planes::Triangle {delta_x: Vec3::new(6., 0., 0.), delta_y: Vec3::new(2., 0., 1.), }

            },
            Object::Plane { pos: Vec3::new(2., 2., 3.), 
                normal: Vec3::new(-0.5, 0., 1.).normalize(), 
                mat: Material {refl: Reflection::Diffuse(), tex: Texture::Solid { color: Vec3::new(0., 1., 0.)}},
                plane_type: Planes::Disk {delta_x: Vec3::new(1., 0., 0.), delta_y: Vec3::new(0., 0.0, 0.5), r: 5.}

            },
            Object::Plane { pos: Vec3::new(0., 0., 0.), 
                normal: Vec3::new(0., 0., 1.), 
                mat: Material {refl: Reflection::Diffuse(), tex: Texture::Solid { color: Vec3::new1(1.)}},
                plane_type: Planes::Plane()
            },
            //Object::Sphere { pos: Vec3::new(0., 0., 3.), rad: 3., mat: Material {tex: Texture::Img {img}, refl: Reflection::Diffuse()} }
        ]
    };

    display(frag, input, "sample");
}
