pub mod geometry;
pub mod render;
pub mod vec3;
pub mod textures;
pub mod objects;
pub use geometry::Camera;
pub use objects::Object;
pub use vec3::Vec3;

pub const WIDTH: usize = 500;
pub const HEIGHT: usize = 500;

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
        let clos = |v: &Vec3| Vec3::lerp(&Vec3::new1(1.),&Vec3::new(0.5, 0.7, 1.), v.dot(&Vec3::up()).max(0.));
        Box::new(clos)
    }
}