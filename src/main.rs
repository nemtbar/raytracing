mod geometry;
mod render;
mod vec3;
mod textures;
#[allow(unused_imports)]
use geometry::{Camera, Material, Object, Reflection};
use rand::Rng;
use render::{display, transform, Pixel};
use textures::Texture;
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
}

impl Default for Uniforms {
    fn default() -> Self {
        Self { sample_count: 100, bounce_count: 50, offset: WIDTH as f32/1000., cam: Camera::default(), objects: vec![] }
    }
    
}

impl Uniforms {
    pub fn new(sample_count: u32, bounce_count: u8, offset: f32, cam: Camera, objects: Vec<Object>) -> Self {
        Self {sample_count, bounce_count, offset, cam, objects}
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
        color_sum = color_sum + Object::bounce(&ray, &input.objects, input.bounce_count);
    }
    //average of color samples
    color_sum = color_sum / input.sample_count as f32;
    transform(color_sum.x, color_sum.y, color_sum.z)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let p1 = Vec3::new(-0.5, -0.2, 0.);
    let p2 = Vec3::new(0.5, -0.2, 0.);
    let p3 = Vec3::new(0., -0.8, 0.);
    let p4 = Vec3::new(0., 0., 1.);
    let mat = Material {refl: Reflection::Metal { roughness: 0. }, tex: Texture::Solid{color: Vec3::new(1., 0.8, 0.)}};
    let objs = vec![
        Object::new_triangle(&p1, &p2, &p3, &mat),
        Object::new_triangle(&p1, &p2, &p4, &mat),
        Object::new_triangle(&p2, &p3, &p4, &mat),
        Object::new_triangle(&p3, &p1, &p4, &mat),
        Object::Sphere { pos: Vec3::new(-1.5, 0., 1.), rad: 1., mat: Material {refl: Reflection::Metal { roughness: 0. }, tex: Texture::Solid{color: Vec3::new1(1.)}} }
    ];
    let input = Uniforms {
        sample_count: 100, 
        bounce_count: 50,
        offset: WIDTH as f32/1000.,
        cam: Camera::new(
            &Vec3::new(0., -2., 1.),
            &Vec3::default(),
            75., 
            &Vec3::new(0., 0., 1.),
            0.
        ),
        objects: vec![
            Object::Plane { pos: Vec3::new(0., 0., 0.), normal: Vec3::new(0., 0., 1.), mat: Material {refl: Reflection::Diffuse(), tex: Texture::Solid{color: Vec3::new(1., 1., 1.)}}},
            Object::BoundBox { min: Vec3::new(-50., -2., -2.), max: Vec3::new(50., 200., 40.), inside: objs }
        ]
    };

    display(frag, input, "sample");
}
