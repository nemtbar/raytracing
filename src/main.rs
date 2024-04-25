mod vec3;
mod render;
mod geometry;
use std::env;
use vec3::{Vec3, Point};
use render::{display, transform, Pixel};
use geometry::{Object, Ray, Material, Reflection};
use rand::Rng;
use std::{self, process::Command};
pub const WIDTH: usize = 890;
pub const HEIGHT: usize = 500;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html



//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize) -> Pixel{
    //uv coordinates between -focal->focal
    let focal = 1.;
    let mut ux = (x as f32) / (WIDTH as f32) * (focal  * 2.)- focal;
    let uy = ((y as f32) / (HEIGHT as f32) * (focal * 2.) - focal) * -1.;
    ux *= WIDTH as f32/HEIGHT as f32;
    let camera: Point = Vec3::new(0., -1.2, 0.);
    let white = Vec3::new(1., 1., 1.);
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(0., 0., 0.), rad: 0.5, mat: Material{color: white.clone(), refl: Reflection::Diffuse()}},
        Object::Plane {pos: Vec3::new(0., 0., -1.), normal: Vec3::new(0., 0., 1.), mat: Material{color: white, refl: Reflection::Diffuse()}},
        //Object::Sphere {pos: Vec3::new(0., 0., -40.), col: Vec3::new(1., 1., 1.), rad: 39., mat: Material::Diffuse()},
        Object::Sphere {pos: Vec3::new(-1., 0., 0.), rad: 0.5, mat: Material{color: Vec3::new(0.8, 0.6, 0.2), refl: Reflection::Metal()}},
        Object::Sphere {pos: Vec3::new(1., 0., 0.), rad: 0.5, mat: Material{color: Vec3::new(0.8, 0.8, 0.8), refl: Reflection::Metal()}}
    ];
    let mut col = Vec3::default();
    let c = 50;
    let b = 50;
    let mut rng = rand::thread_rng();
    let offset: f32 = 0.001;
    for i in 0..c{
        let rand_x = rng.gen_range(-offset..offset);
        let rand_y = rng.gen_range(-offset..offset);
        let dir = Vec3::new(ux+rand_x, 1., uy+rand_y).normalize();
        let ray = Ray::new(camera.clone(), dir.clone());
        if i == 0{
            col = Object::bounce(&ray, &objects, b);
        } else {
            col = Object::bounce(&ray, &objects, b).lerp(&col, 1.-1./c as f32);
        }
    }
    transform(col.x, col.y, col.z)

}

fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    display(frag, "sample");
    Command::new("fim").arg("sample.png").spawn().expect("failed to open image with \"fim\" package");
}