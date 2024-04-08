mod vec3;
mod render;
mod geometry;
use vec3::{Vec3, Point};
use render::{Pixel, Image};
use geometry::{Object, Ray};
use std::env;

//https://raytracing.github.io/books/RayTracingInOneWeekend.html



//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize, width: usize, height: usize) -> Pixel{
    //uv coordinates between -g->g
    let g = 1.;
    let ux = (x as f32) / (width as f32) * (g  * 2.)- g;
    let uy = ((y as f32) / (height as f32) * (g * 2.) - g) * -1.;
    //define important points
    let camera: Point = Vec3::new(0., -5., 0.);
    let dir = (Vec3::new(ux, -4., uy)-&camera).normalize();
    let ray = Ray::new(camera.clone(), dir.clone());
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(-3., 1., 0.), col: Vec3::new(1., 1., 1.), rad: 1., emmision: 0.},
        Object::Plane { pos: Vec3::new(0., 0., -1.5), normal: Vec3::new(0., 0., 1.), col: Vec3::new(1., 1., 1.)},
        //Object::Sphere {pos: Vec3::new(0., 0., -40.), col: Vec3::new(1., 1., 1.), rad: 39., emmision: 0.},
        Object::Sphere {pos: Vec3::new(0., 0., 1.), col: Vec3::new(1., 1., 1.), rad: 2., emmision: 0.3}
    ];
    let mut col = Pixel::default();
    let mut atr: [u8; 2] = [20, 30];
    let arguments = env::args().collect::<Vec<String>>();
    for i in 1..arguments.len().min(atr.len()+1){
        let num = arguments[i].parse::<u8>().expect("cannot convert argument to valid number");
        atr[i-1] = num;
    }
    for i in 0..atr[0]{
        if i == 0{
            col = Object::bounce(ray.clone(), &objects, atr[1]);
        } else {
            col = Object::bounce(ray.clone(), &objects, atr[1]).lerp(&col, 1.- 1./(atr[0] as f32));
        }
    }
    col

}

fn main(){
    let sample = Image::new_with_method(frag);
    sample.display().unwrap(); 
}