#![allow(dead_code)]

mod geometry;
mod render;
mod vec3;
use geometry::{Camera, Material, Object, Reflection};
use rand::Rng;
use render::{display, transform, Pixel};
use std::env;
use vec3::Vec3;
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html


mod uniforms {
    use lazy_static::lazy_static;

    use super::*;
    pub const FOCAL: f32 = 0.7;
    pub const SAMPLE_COUNT: u32 = 30;
    pub const BOUNCE_COUNT: u8 = 50;
    pub const OFFSET: f32 = WIDTH as f32 / 1000.;
    pub const WHITE: Vec3 = Vec3::new(1., 1., 1.);
    pub const OBJECTS: [Object; 3] = [
    Object::Sphere {
        pos: Vec3::new(0., 0., 0.),
        rad: 1.,
        mat: Material {
            color: Vec3::new(1., 0.78, 0.),
            refl: Reflection::Diffuse(),
        },
    },
    Object::Plane { 
        pos: Vec3::new(0., 0., -1.),
        normal: Vec3::new(0., 0., 1.),
        mat: Material {
            color: WHITE,
            refl: Reflection::Diffuse(),
        },
    },
    Object::Sphere { 
        pos: Vec3::new(-2., 0., 0.),
        rad: 1.,
        mat: Material {
            color: WHITE,
            refl: Reflection::Metal { roughness: 0.1 },
        },
    }

    ];
    lazy_static!(
        pub static ref cam: Camera = Camera::new(
        &Vec3::new(-5., -4., 2.),
        &Vec3::new(0., 0., 0.),
        90.,
        &Vec3::new(0., 0., 1.), //up
        );
    );

    
}
//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize) -> Pixel {
    use uniforms::*;
    let mut rng = rand::thread_rng();
    let mut color_sum = Vec3::default();
    for _ in 0..SAMPLE_COUNT {
        let rand_x = rng.gen_range(-OFFSET..OFFSET);
        let rand_y = rng.gen_range(-OFFSET..OFFSET);
        let ray = cam.shoot(x as f32 + rand_x, y as f32 + rand_y);
        color_sum = color_sum + Object::bounce(&ray, &OBJECTS, BOUNCE_COUNT);
    }
    //average of color samples
    color_sum = color_sum / SAMPLE_COUNT as f32;
    transform(color_sum.x, color_sum.y, color_sum.z)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    display(frag, "sample");
}
