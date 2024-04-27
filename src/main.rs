mod vec3;
mod render;
mod geometry;
use vec3::{Vec3, Point};
use render::{display, Pixel};
use geometry::{Object, Ray};
pub const WIDTH: usize = 700;
pub const HEIGHT: usize = 700;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html
//https://www.youtube.com/watch?v=Qz0KTGYJtUk&t=892s

//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize) -> Pixel{
    //uv coordinates between -g->g
    let g = 1.;
    let ux = (x as f32) / (WIDTH as f32) * (g  * 2.)- g;
    let uy = ((y as f32) / (HEIGHT as f32) * (g * 2.) - g) * -1.;
    //define important points
    let camera: Point = Vec3::new(0., -3., 0.);
    let dir = (Vec3::new(ux, camera.y+1., uy)-&camera).normalize();
    //camera = camera.rot_x(-20.);
    //dir = dir.rot_x(-20.);
    let ray = Ray::new(camera.clone(), dir.clone());
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(0., 0., 0.), col: Vec3::new(1., 0., 0.), rad: 1., emmision: 0.},
        Object::Plane {pos: Vec3::new(0., 0., -1.), normal: Vec3::new(0., 0., 1.), col: Vec3::new(1., 1., 1.), emmision: 0.},
        //Object::Sphere {pos: Vec3::new(0., 0., -40.), col: Vec3::new(1., 1., 1.), rad: 39., emmision: 0.},
        Object::Sphere {pos: Vec3::new(0., 0., 8.7), col: Vec3::new(1., 0., 0.), rad: 5., emmision: 3.0},
        //Object::Plane {pos: Vec3::new(0., 0., 20.), normal: Vec3::new(0., 0., -1.), col: Vec3::new(1., 1., 1.), emmision: 2.}
    ];
    let mut col = Vec3::default();
    let c = 100;
    let max_b = 30;
    for _ in 0..c{
        col = col + Object::bounce(ray.clone(), &objects, max_b)
    }
    col = col / c as f32;
    Pixel::new((col.x * 255.) as u8, (col.y * 255.) as u8, (col.z * 255.) as u8)

}
fn main(){
    display(frag, "sample");
}