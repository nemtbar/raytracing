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
    pub const OFFSET: f32 = 0.001;
    pub const WHITE: Vec3 = Vec3::new(1., 1., 1.);
    pub const OBJECTS: [Object; 5] = [
    Object::Sphere {
        pos: Vec3::new(2.1, 0., 0.),
        rad: 1.,
        mat: Material {
            color: Vec3::new(1., 0.78, 0.),
            refl: Reflection::Metal { roughness: 0.1 },
        },
    },
    Object::Sphere{
        pos: Vec3::new(0., 0., -50.),
        rad: 49.,
        mat: Material {
            color: Vec3::new(1., 0.984, 0.),
            refl: Reflection::Diffuse(),
        },
    },
    Object::Sphere {
        pos: Vec3::new(-2.1, 0., 0.), 
        rad: 1., 
        mat: Material{
            color: WHITE, 
            refl: Reflection::Glass { reflective: 1.5 }
        }
    },
    Object::Sphere {
        pos: Vec3::new(-2.1, 0., 0.), 
        rad: 0.5, 
        mat: Material{
            color: WHITE, 
            refl: Reflection::Glass { reflective: 1. }
        }
    },
    Object::Sphere {
        pos: Vec3::new(0., 0., 0.), 
        rad: 1., 
        mat: Material{
            color: Vec3::new(0.1, 0.1, 1.),
            refl: Reflection::Diffuse()
        }
    }

    ];
    lazy_static!(
        pub static ref cam: Camera = Camera::new(Vec3::new(0., -45., 90.), 10.);
    );

    
}
//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize) -> Pixel {
    //uv coordinates between -FOCAL->FOCAL
    use uniforms::*;
    let mut ux = (x as f32) / (WIDTH as f32) * (FOCAL * 2.) - FOCAL;
    let uy = ((y as f32) / (HEIGHT as f32) * (FOCAL * 2.) - FOCAL) * -1.;
    ux *= WIDTH as f32 / HEIGHT as f32;

    let mut rng = rand::thread_rng();
    let mut color_sum = Vec3::default();
    for _ in 0..SAMPLE_COUNT {
        let rand_x = rng.gen_range(-OFFSET..OFFSET);
        let rand_y = rng.gen_range(-OFFSET..OFFSET);
        let ray = cam.shoot(ux + rand_x, uy + rand_y);
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
