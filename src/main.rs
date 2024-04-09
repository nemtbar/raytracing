mod vec3;
mod render;
mod geometry;
use vec3::{Vec3, Point};
use render::{Pixel, Image};
use geometry::{Object, Ray};

//https://raytracing.github.io/books/RayTracingInOneWeekend.html



//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize, width: usize, height: usize) -> Pixel{
    //uv coordinates between -g->g
    let g = 1.;
    let ux = (x as f32) / (width as f32) * (g  * 2.)- g;
    let uy = ((y as f32) / (height as f32) * (g * 2.) - g) * -1.;
    //define important points
    let mut camera: Point = Vec3::new(0., -5., 0.);
    let mut dir = (Vec3::new(ux, camera.y+1., uy)-&camera).normalize();
    //camera = camera.rot_x(-20.);
    //dir = dir.rot_x(-20.);
    let ray = Ray::new(camera.clone(), dir.clone());
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(-3., 1., 0.7), col: Vec3::new(1., 1., 1.), rad: 1., emmision: 0.},
        Object::Plane {pos: Vec3::new(0., 0., -0.5), normal: Vec3::new(0., 0., 1.), col: Vec3::new(1., 1., 1.)},
        //Object::Sphere {pos: Vec3::new(0., 0., -40.), col: Vec3::new(1., 1., 1.), rad: 39., emmision: 0.},
        Object::Sphere {pos: Vec3::new(0., 0., 1.7), col: Vec3::new(1., 1., 1.), rad: 2., emmision: 0.3}
    ];
    let mut col = Pixel::default();
    let c = 10;
    let max_b = 30;
    for i in 0..c{
        if i == 0{
            col = Object::bounce(ray.clone(), &objects, max_b);
        } else {
            col = Object::bounce(ray.clone(), &objects, max_b).lerp(&col, 1.- 1./(c as f32));
        }
    }
    col

}

fn main(){
    let sample = Image::new_with_method(frag);
    sample.display().unwrap(); 
}