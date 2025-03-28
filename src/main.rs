#![allow(unused_imports)]

use raytracing::{
    Uniforms,
    geometry::*,
    render::*,
    vec3::Vec3,
    textures::*,
    objects::*,
};

mod scenes;
use scenes::{lalaland, cylinder_test};
use image::RgbImage;
use image::{ImageBuffer, ImageReader};
use rand::Rng;
use std::env;


//https://raytracing.github.io/books/RayTracingInOneWeekend.html


//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize, input: &Uniforms) -> Pixel {
    let mut rng = rand::thread_rng();
    let mut color_sum = Vec3::default();
    for _ in 0..input.sample_count {
        let mut rand_x: f32 = 0.;
        let mut rand_y: f32 = 0.;
        if input.offset != 0. {
            rand_x = rng.gen_range(-input.offset..input.offset);
            rand_y = rng.gen_range(-input.offset..input.offset);
        }
        let ray = input.cam.shoot(x as f32 + rand_x, y as f32 + rand_y);
        color_sum = color_sum + Object::bounce(&ray, &input.objects, input.bounce_count, &input.env_shader);
    }
    //average of color samples
    let avg_color = color_sum / input.sample_count as f32;
    Pixel::from_vec(avg_color)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut uni = scenes::cylinder_test();
    uni.sample_count = 300;
    let pic = display(frag, uni);
    pic.to_buffer().save("sample2.png").expect("an error occured while saving the image");
    
}
