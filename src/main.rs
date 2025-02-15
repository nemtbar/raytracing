mod geometry;
mod render;
mod vec3;
mod readin;
#[allow(unused_imports)]
use geometry::{Camera, Material, Object, Reflection};
use rand::Rng;
use render::{display, transform, Pixel};
use std::env;
use vec3::Vec3;
use serde::{Deserialize, Serialize};
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html

#[derive(Deserialize, Serialize)]
pub struct Uniforms {
    pub sample_count: u32,
    pub bounce_count: u8,
    pub offset: f32,
    pub cam: Camera,
    pub objects: Vec<Object>,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self { sample_count: 100, bounce_count: 50, offset: WIDTH as f32/1000., cam: Camera::default(), objects: readin::safe_load_objects("objects.json", vec![]) }
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
    let mut input = Uniforms {
        sample_count: 700, 
        bounce_count: 50,
        offset: WIDTH as f32/1000.,
        cam: Camera::new(
            &Vec3::new(0., -20., 4.),
            &Vec3::default(),
            75., 
            &Vec3::new(0., 0., 1.),
            0.
        ),
        objects: vec![
            Object::Plane { pos: Vec3::new(0., 0., -1.), normal: Vec3::new(0., 0., 1.), mat: Material {color: Vec3::new(1., 1., 1.), refl: Reflection::Diffuse()}},
        ]
    };
    for _ in 0..30 {
        let r = Vec3::random();
        let sph = Object::Sphere { pos: Vec3::new(r.x, r.y, 0.) * 20., rad: 1., mat: Material {color: Vec3::new(1., 0., 0.).lerp(&Vec3::new(0., 0., 1.), r.z.abs()), refl: Reflection::Diffuse()}};
        input.objects.push(sph);
    }
    for _ in 0..10 {
        let r = Vec3::random();
        let glass = Object::Sphere { pos: Vec3::new(r.x, r.y, 0.) * 10., rad: 1., mat: Material {color: Vec3::new(1., 1., 1.), refl: Reflection::Glass { reflective: 1.5 }}};
        input.objects.push(glass);
    }
    for _ in 0..5 {
        let r = Vec3::random();
        let sph = Object::Sphere { pos: Vec3::new(r.x, r.y, 0.) * 10., rad: 1., mat: Material {color: Vec3::new(1., 1., 1.), refl: Reflection::Metal { roughness: 0. }}};
        input.objects.push(sph);
    }
    display(frag, input, "sample");
}
