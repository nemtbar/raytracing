mod vec3;
mod render;
mod geometry;
use vec3::{Vec3, Point};
use render::{display, transform, Pixel};
use geometry::{Object, Ray, Material};
use rand::Rng;
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html



//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize) -> Pixel{
    //uv coordinates between -g->g
    let g = 1.;
    let mut ux = (x as f32) / (WIDTH as f32) * (g  * 2.)- g;
    let uy = ((y as f32) / (HEIGHT as f32) * (g * 2.) - g) * -1.;
    ux *= WIDTH as f32/HEIGHT as f32;
    let camera: Point = Vec3::new(0., -5., 0.);
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(-2., 0., 0.), col: Vec3::new(1., 1., 1.), rad: 1., mat: Material::Diffuse()},
        Object::Plane {pos: Vec3::new(0., 0., -1.), normal: Vec3::new(0., 0., 1.), col: Vec3::new(1., 1., 1.), mat: Material::Diffuse()},
        //Object::Sphere {pos: Vec3::new(0., 0., -40.), col: Vec3::new(1., 1., 1.), rad: 39., mat: Material::Diffuse()},
        Object::Sphere {pos: Vec3::new(0., 0., 0.0), col: Vec3::new(1., 1., 1.), rad: 1., mat: Material::Reflective()}
    ];
    let mut col = Vec3::default();
    let c = 10;
    let b = 30;
    let mut rng = rand::thread_rng();
    let offset: f32 = 0.002;
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
    //env::set_var("RUST_BACKTRACE", "1");
    display(frag)
}