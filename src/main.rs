mod vec3;
mod render;
mod geometry;
use vec3::{Vec3, Point};
use render::{Pixel, display};
use geometry::{Object, Ray};
use rand::Rng;
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html



//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize) -> Pixel{
    //uv coordinates between -g->g
    let g = 1.;
    let ux = (x as f32) / (WIDTH as f32) * (g  * 2.)- g;
    let uy = ((y as f32) / (HEIGHT as f32) * (g * 2.) - g) * -1.;
    let camera: Point = Vec3::new(0., -5., 0.);
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(-3., 1., 0.), col: Vec3::new(1., 1., 1.), rad: 1.},
        Object::Plane {pos: Vec3::new(0., 0., -2.), normal: Vec3::new(0., 0., -1.), col: Vec3::new(1., 1., 1.)},
        //Object::Sphere {pos: Vec3::new(0., 0., -40.), col: Vec3::new(1., 1., 1.), rad: 39.},
        Object::Sphere {pos: Vec3::new(0., 0., 0.2), col: Vec3::new(1., 1., 1.), rad: 1.}
    ];
    let mut col = Pixel::default();
    let c = 30;
    let b = 30;
    let mut rng = rand::thread_rng();
    let offset: f32 = 0.001;
    let rand_x = rng.gen_range(-offset..offset);
    let rand_y = rng.gen_range(-offset..offset);
    let dir = Vec3::new(ux+rand_x, 1., uy+rand_y).normalize();
    for i in 0..c{

        let ray = Ray::new(camera.clone(), dir.clone());
        if i == 0{
            col = Object::bounce(ray.clone(), &objects, b);
        } else {
            col = Object::bounce(ray.clone(), &objects, b).lerp(&col, 1.-1./c as f32);
        }
    }
    col

}

fn main(){
    display(frag)
}