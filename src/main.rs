mod geometry;
mod render;
mod vec3;
mod textures;
use image::RgbImage;
#[allow(unused_imports)]
use image::{ImageBuffer, ImageReader};
use geometry::{Camera, Material, Object, QuadType, Reflection};
use rand::Rng;
use render::{display, transform, Pixel};
use textures::{Texture, Picture};
use std::env;
use vec3::Vec3;
pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;
//https://raytracing.github.io/books/RayTracingInOneWeekend.html

pub struct Uniforms {
    pub sample_count: u32,
    pub bounce_count: u8,
    pub offset: f32,
    pub cam: Camera,
    pub objects: Vec<Object>,
    pub env_shader: Box<dyn Fn(&Vec3) ->Vec3+Send+Sync>
}

impl Default for Uniforms {
    fn default() -> Self {
        let func = |v: &Vec3| Vec3::lerp(&Vec3::new1(1.),&Vec3::new(0.5, 0.5, 0.95), (v.dot(&(Vec3::up()*-1.)).max(0.)).abs());
        Self { sample_count: 100, bounce_count: 50, offset: WIDTH as f32/1000., cam: Camera::default(), objects: vec![], env_shader: Box::new(func) }
    }
}    


impl Uniforms {
    pub fn new(sample_count: u32, bounce_count: u8, offset: f32, cam: Camera, objects: Vec<Object>, env_shader: Box<dyn Fn(&Vec3)->Vec3 + Send + Sync>) -> Self {
        Self {sample_count, bounce_count, offset, cam, objects, env_shader}
    }
    pub fn get_env_shader() -> Box<dyn Fn(&Vec3)->Vec3+Sync+Send>{
        let clos = |v: &Vec3| Vec3::lerp(&Vec3::new1(1.),&Vec3::new(0.5, 0.5, 0.95), (v.dot(&(Vec3::up()*-1.)).max(0.)).abs());
        Box::new(clos)
    }
}

//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize, input: &Uniforms) -> Pixel {
    let mut rng = rand::thread_rng();
    let mut color_sum = Vec3::default();
    for _ in 0..input.sample_count {
        let rand_x = rng.gen_range(-input.offset..input.offset);
        let rand_y = rng.gen_range(-input.offset..input.offset);
        let ray = input.cam.shoot(x as f32 + rand_x, y as f32 + rand_y);
        color_sum = color_sum + Object::bounce(&ray, &input.objects, input.bounce_count, &input.env_shader);
    }
    //average of color samples
    let avg_color = color_sum / input.sample_count as f32;
    transform(avg_color.x, avg_color.y, avg_color.z)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let image: RgbImage = ImageReader::open("earthmap.jpg").unwrap().decode().unwrap().into_rgb8();
    let _img = Picture::new(image);
    let mat = Material::new(Reflection::Diffuse(), Texture::Solid { color: Vec3::new1(1.) }, Vec3::new1(4.));
    let input = Uniforms {
        sample_count: 100, 
        bounce_count: 5,
        offset: WIDTH as f32/1000.,
        cam: Camera::new(
            &Vec3::new(0., -2., 3.3),
            &Vec3::new(0., 0., 3.),
            95.,
            &Vec3::up(),
            0.
        ),
        objects: vec![
            Object::Plane { pos: Vec3::default(), normal: Vec3::up(), mat: Material::default() }
            //Object::Sphere { pos: Vec3::new(0., 4.5, 1.5), rad: 1.5, mat: Material{refl: Reflection::Metal { roughness: 0. }, tex: Texture::Solid { color: Vec3::new1(1.) }, emmision: Vec3::default()} },
            // Object::new_quad(Vec3::new(-3., 0., 0.), Vec3::new(6., 0., 0.), Vec3::new(0., 6., 0.), QuadType::Rect(), Material::default()),
            // Object::new_quad(Vec3::new(-3., 0., 6.), Vec3::new(6., 0., 0.), Vec3::new(0., 6., 0.), QuadType::Rect(), Material::default()),

            // Object::new_quad(Vec3::new(-3., 0., 0.), Vec3::new(0., 6., 0.), Vec3::new(0., 0., 6.), QuadType::Rect(), Material{refl: Reflection::Diffuse(), tex: Texture::Solid { color: Vec3::new(1., 0.1, 0.1) }, emmision: Vec3::default()}),
            // Object::new_quad(Vec3::new(3., 0., 0.), Vec3::new(0., 6., 0.), Vec3::new(0., 0., 6.), QuadType::Rect(), Material{refl: Reflection::Diffuse(), tex: Texture::Solid { color: Vec3::new(0.1, 1.0, 0.1) }, emmision: Vec3::default()}),

            // Object::new_quad(Vec3::new(-3., 6., 0.), Vec3::new(6., 0., 0.), Vec3::new(0., 0., 6.), QuadType::Rect(), Material::default()),
            // Object::new_quad(Vec3::new(-1.5, 1.5, 5.98), Vec3::new(3., 0., 0.), Vec3::new(0., 3., 0.), QuadType::Rect(), mat),

            ],
        env_shader: Box::new(|v|{
            let col1 = Vec3::new(0.3843, 0.1294, 0.702);
            let col2 = Vec3::new(0.018, 0.0157, 0.2039);
            let value = v.dot(&Vec3::up()).max(0.);
            col1.lerp(&col2, value)
        } )
    };

    display(frag, input, "sample");
}
