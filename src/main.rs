mod vec3;
mod render;
mod geometry;
use rand::Rng;
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
    let mut blur = Pixel::default();
    //define important points
    let camera: Point = Vec3::new(0., -5., 0.);
    let mut rng = rand::thread_rng();
    for index in 0..5{
        let rand_x = rng.gen_range(-0.001..0.001);
        let rand_y = rng.gen_range(-0.001..0.001);
        let dir = (Vec3::new(ux+rand_x, &camera.y+1., uy+rand_y)-&camera).normalize();
        let ray = Ray::new(camera.clone(), dir.clone());
        let objects: Vec<Object> = vec![
            Object::Sphere {pos: Vec3::new(-3., 1., 0.), col: Vec3::new(1., 0., 0.), rad: 1., emmision: 0.},
            Object::Sphere {pos: Vec3::new(0., 0., -40.), col: Vec3::new(0.35, 0.87, 0.35), rad: 39., emmision: 0.},
            Object::Sphere {pos: Vec3::new(0., 1., 0.), col: Vec3::new(0., 0., 1.), rad: 2., emmision: 3.}
        ];

        if index == 0 {
            blur = Object::bounce(ray.clone(), &objects, 2);
        }else {
            let col = Object::bounce(ray.clone(), &objects, 2);
            blur = blur.lerp(&col, 1./5.);
        }
        
    }
    blur
}

fn main(){
    let sample = Image::new_with_method(frag);
    sample.display().unwrap(); 
}