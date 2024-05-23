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

//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize) -> Pixel {
    //uv coordinates between -focal->focal
    let focal = 0.7;
    let mut ux = (x as f32) / (WIDTH as f32) * (focal * 2.) - focal;
    let uy = ((y as f32) / (HEIGHT as f32) * (focal * 2.) - focal) * -1.;
    ux *= WIDTH as f32 / HEIGHT as f32;

    let white = Vec3::new(1., 1., 1.);
    let objects: Vec<Object> = vec![
        Object::Sphere {
            pos: Vec3::new(0., 1.6, 0.),
            rad: 0.5,
            mat: Material {
                color: Vec3::new(0.1, 0.2, 0.9),
                refl: Reflection::Diffuse(),
            },
        },
        Object::Plane {
            pos: Vec3::new(0., 0., -1.),
            normal: Vec3::new(0., 0., 1.),
            mat: Material {
                color: white.clone(),
                refl: Reflection::Diffuse(),
            },
        },
        //Object::Sphere {pos: Vec3::new(0., 0., -40.), rad: 39., mat: Material{color: Vec3::new(1., 1., 0.), refl: Reflection::Diffuse()}},
        Object::Sphere {
            pos: Vec3::new(0., 0., 0.2),
            rad: 1.,
            mat: Material {
                color: white.clone(),
                refl: Reflection::Glass()
            },
        },
    ];
    let c = 10;
    let b = 10;
    let mut rng = rand::thread_rng();
    let offset: f32 = 0.001;
    let mut color_sum = Vec3::default();
    let cam = Camera::new(Vec3::default(), Vec3::new(0., 0., 0.), 3.);
    for _ in 0..c {
        let rand_x = rng.gen_range(-offset..offset);
        let rand_y = rng.gen_range(-offset..offset);
        let ray = cam.shoot(ux + rand_x, uy + rand_y);
        color_sum = color_sum + Object::bounce(&ray, &objects, b);
    }
    //average of color samples
    color_sum = color_sum / c as f32;
    transform(color_sum.x, color_sum.y, color_sum.z)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    display(frag, "sample");
}
