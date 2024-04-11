mod vec3;
mod render;
mod geometry;
use vec3::{Vec3, Point};
use render::{Pixel, Image};
use geometry::{Object, Ray};
use rand::Rng;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html



//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize, width: usize, height: usize) -> Pixel{
    //uv coordinates between -g->g
    let g = 1.;
    let ux = (x as f32) / (width as f32) * (g  * 2.)- g;
    let uy = ((y as f32) / (height as f32) * (g * 2.) - g) * -1.;
    let mut camera: Point = Vec3::new(0., -5., 0.);
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(-3., 1., 0.), col: Vec3::new(1., 1., 1.), rad: 1.},
        Object::Plane { pos: Vec3::new(0., 0., -1.5), normal: Vec3::new(0., 0., -1.), col: Vec3::new(1., 1., 1.)},
        //Object::Sphere {pos: Vec3::new(0., 0., -40.), col: Vec3::new(1., 1., 1.), rad: 39.},
        Object::Sphere {pos: Vec3::new(0., 0., 0.0), col: Vec3::new(1., 0., 1.), rad: 2.}
    ];
    let mut col = Pixel::default();
    let c = 30;
    let b = 30;
    let mut rng = rand::thread_rng();
    let offset: f32 = 0.002;
    let rand_x = rng.gen_range(-offset..offset);
    let rand_y = rng.gen_range(-offset..offset);
    let mut dir = Vec3::new(ux+rand_x, 1., uy+rand_y).normalize();
    camera = camera.rot_x(-20.);
    dir = dir.rot_x(-20.);
    camera = camera.rot_z(90.);
    dir = dir.rot_z(90.);
    for i in 0..c{

        let ray = Ray::new(camera.clone(), dir.clone());
        if i == 0{
            col = Object::bounce(ray.clone(), &objects, b);
        } else {
            col = Object::bounce(ray.clone(), &objects, b).lerp(&col, 0.5);
        }
    }
    col

}

fn main(){
    let sample = Image::new_with_method(frag);
    sample.display().unwrap(); 
}