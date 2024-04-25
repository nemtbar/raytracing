mod vec3;
mod render;
mod geometry;
use std::env;
use vec3::{Vec3, Point};
use render::{display, transform, Pixel};
use geometry::{Object, Ray, Material, Reflection};
use rand::Rng;
use std::{self, process::Command};
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html

const CAMERA: Point = Vec3::new(0., -3., 0.);



//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize) -> Pixel{
    //uv coordinates between -focal->focal
    let focal = 1.;
    let mut ux = (x as f32) / (WIDTH as f32) * (focal  * 2.)- focal;
    let uy = ((y as f32) / (HEIGHT as f32) * (focal * 2.) - focal) * -1.;
    ux *= WIDTH as f32/HEIGHT as f32;
    
    let white = Vec3::new(1., 1., 1.);
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(2.1, 0., 0.), rad: 1., mat: Material{color: white.clone(), refl: Reflection::Diffuse()}},
        Object::Plane {pos: Vec3::new(0., 0., -1.), normal: Vec3::new(0., 0., 1.), mat: Material{color: white.clone(), refl: Reflection::Diffuse()}},
        //Object::Sphere {pos: Vec3::new(0., 0., -40.), rad: 39., mat: Material{color: Vec3::new(1., 1., 0.), refl: Reflection::Diffuse()}},
        Object::Sphere {pos: Vec3::new(0., 0.2, 0.1), rad: 1., mat: Material{color: Vec3::new(1., 0.7, 0.7), refl: Reflection::Metal()}},
    ];
    let c = 100;
    let b = 50;
    let mut rng = rand::thread_rng();
    let offset: f32 = 0.001;
    let mut color_sum = Vec3::default();
    for _ in 0..c{
        let rand_x = rng.gen_range(-offset..offset);
        let rand_y = rng.gen_range(-offset..offset);
        let dir = Vec3::new(ux+rand_x, 1., uy+rand_y).normalize();
        let ray = Ray::new(CAMERA.clone(), dir.clone());
        color_sum = color_sum + Object::bounce(&ray, &objects, b);
    }
    //average of color samples
    color_sum = color_sum / c as f32;
    transform(color_sum.x, color_sum.y, color_sum.z)

}

fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    display(frag, "sample");
    Command::new("fim").arg("sample.png").spawn().expect("failed to open image with \"fim\" package");
}