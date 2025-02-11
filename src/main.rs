mod geometry;
mod render;
mod vec3;
mod readin;
use geometry::{Camera, Material, Object, Reflection};
use rand::Rng;
use render::{display, transform, Pixel};
use std::env;
use vec3::Vec3;
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html


mod uniforms {
    use lazy_static::lazy_static;
    use super::*;
    pub const SAMPLE_COUNT: u32 = 20;
    pub const BOUNCE_COUNT: u8 = 50;
    pub const OFFSET: f32 = WIDTH as f32 / 1000.;
    lazy_static!(
        pub static ref cam: Camera = Camera::new(
            &Vec3::new(0., -5., 4.),
            &Vec3::new(0., 0., 0.),
            90.,
            //must not be parallel to the lookfrom-lookat vector
            &Vec3::new(0., 0., 1.), //up
            0.,
        );
        pub static ref OBJECTS: Vec<Object> = readin::safe_load_objects("objects.json", vec![]);
    );


    
}
//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize) -> Pixel {
    use uniforms::*;
    let mut rng = rand::thread_rng();
    let mut color_sum = Vec3::default();
    for _ in 0..SAMPLE_COUNT {
        let rand_x = rng.gen_range(-OFFSET..OFFSET);
        let rand_y = rng.gen_range(-OFFSET..OFFSET);
        let ray = cam.shoot(x as f32 + rand_x, y as f32 + rand_y);
        color_sum = color_sum + Object::bounce(&ray, &OBJECTS, BOUNCE_COUNT);
    }
    //average of color samples
    color_sum = color_sum / SAMPLE_COUNT as f32;
    transform(color_sum.x, color_sum.y, color_sum.z)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    display(frag, "sample");
}
