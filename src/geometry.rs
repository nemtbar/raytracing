use crate::{vec3::{Point, Vec3}, WIDTH, HEIGHT, textures::Texture};
use rand::Rng;
pub struct HitInfo {
    pub p: Point,
    pub normal: Vec3,
    pub material: Material,
    pub u: f32,
    pub v: f32
}

impl HitInfo {
    pub fn get_color(&self, future: Vec3) -> Vec3{
        let fin_color: Vec3;
        match &self.material.tex {
            Texture::Solid { color } => fin_color = color.clone(),
            Texture::Checker{color1, color2, size} => {
                let x = (self.p.x / size).round() as i32;
                let y = (self.p.y / size).round() as i32;
                let z = (self.p.z / size).round() as i32;

                if (x+y+z)%2 == 0 {
                    fin_color = color1.clone();
                } else {
                    fin_color = color2.clone();
                }
            }
            Texture::Img{img} => {
                assert!(self.u >= 0. && self.u <= 1. && self.v >= 0. && self.v <= 1., "texture coord not in 0-1");
                let x = (self.u * (img.width as f32)) as usize;
                let y = (self.v * (img.height as f32)) as usize;
                let color = img.get_pixel(x, y);
                fin_color = color;
            }
        }
        &self.material.emmision + fin_color * future * 0.9 
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Reflection {
    Diffuse(),
    //roughness is normalized
    Metal{roughness: f32},
    Glass{reflective: f32}
}


#[derive(Clone, Debug)]
pub struct Material {
    pub refl: Reflection,
    pub tex: Texture,
    pub emmision: Vec3
}

impl Material {
    pub fn new(refl: Reflection, tex: Texture, emmision: Vec3)-> Self {
        Self { refl, tex, emmision }
    }
}

impl Default for Material{
    fn default() -> Self {
        Self{
            refl: Reflection::Diffuse(),
            tex: Texture::Solid { color: Vec3::new1(1.) },
            emmision: Vec3::default()
        }
    }
}
fn reflectance(cos: f32, eta: f32) -> f32 {
    //Schlick's approximation
    let r0 = ((1. - eta) / (1. + eta)).powi(2);
    r0 + (1. - r0) * (1. - cos).powi(5)

}

//the eta is the ratio of the refractive index of the second medium to the first medium
//index1*sin(theta1) = index2*sin(theta2)
fn snell(incoming: &Vec3, normal: &Vec3, eta: f32) -> Vec3 {
    let cos_theta = (-1. * incoming).dot(normal).min(1.);
    let refl = reflectance(cos_theta.abs(), eta) > rand::thread_rng().gen_range(0.0..1.0);
    //sin^2theta + cos^2theta = 1
    // total internal reflection
    if (eta * (1.- cos_theta * cos_theta).abs().sqrt()) > 1. || refl{
        return (incoming + normal * 2.).normalize()
    }
    let r_out_perp: Vec3 = eta * &(incoming + normal * cos_theta);
    let r_out_parallel: Vec3 = (1.-r_out_perp.length_squared()).abs().sqrt() * -1. * normal;
    return (r_out_parallel + r_out_perp).normalize();
}

pub fn scatter(ray: &Ray, hit: &HitInfo) -> Ray {
    let mut sol: Ray = Ray::new(hit.p.clone(), Vec3::new(0., 0., 0.));
    match hit.material.refl {
        Reflection::Diffuse() => {
            //lambertian reflection
            let poi = &hit.p + &hit.normal;
            sol.dir = (poi+Vec3::random() - &hit.p).normalize();

        } 
        Reflection::Metal{roughness} => {
            sol.dir = &ray.dir + &hit.normal * 2.;
            sol.dir = sol.dir.normalize() + Vec3::random() * roughness;
            sol.dir = sol.dir.normalize();
        }
        Reflection::Glass{reflective} => {
            if ray.dir.dot(&hit.normal) > 0.{
                //backface
                sol.dir = snell(&ray.dir, &(-1. * &hit.normal), reflective);
            } else {
                sol.dir = snell(&ray.dir, &hit.normal, 1./reflective);

            }
            assert!(sol.dir.is_normalized(), "scatter glass");
        }

    }
    sol
}

#[derive(Clone)]
pub struct Ray {
    pub start: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(start: Point, dir: Vec3) -> Self{
        Self{start, dir}
    }
}

pub struct Camera {
    start: Vec3,
    upper_left: Point,
    delta_x: Vec3,
    delta_y: Vec3,
    blur: f32
}


impl Camera {
    pub fn new(lookfrom: &Point, lookat: &Point, vertical_fov: f32, up: &Vec3, blur: f32) -> Self {
        assert!(up.is_normalized(), "up vector must be normalized at Camera::new");
        let focal_length = (lookfrom - lookat).length();
        let theta = vertical_fov.to_radians();
        let h = (theta/2.).tan();

        let viewport_height = 2. * h * focal_length;
        let viewport_width = WIDTH as f32 / HEIGHT as f32 * viewport_height;

        let w = (lookfrom - lookat).normalize();
        assert!(w.dot(&up).abs() < 0.999, "up vector must not be parallel to the lookfrom-lookat vector\nat Camera::new");
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = -1. * &v * viewport_height;
        let pixel_delta_u = &viewport_u / WIDTH as f32;
        let pixel_delta_v = &viewport_v / HEIGHT as f32;

        let upper_left: Vec3 = lookat - (viewport_u / 2.) - (viewport_v / 2.);
        Self { start: lookfrom.clone(), upper_left, delta_x: pixel_delta_u, delta_y: pixel_delta_v, blur }
    }

    pub fn shoot(&self, ux: f32, uy: f32) -> Ray {
        let mut count = 0;
        let mut rand_disk_x: f32;
        let mut rand_disk_y: f32;
        loop {
            //needed for focus and blur
            rand_disk_x = rand::thread_rng().gen_range(-1.0..1.0);
            rand_disk_y = rand::thread_rng().gen_range(-1.0..1.0);
            if rand_disk_x * rand_disk_x + rand_disk_y * rand_disk_y <= 1. || count > 50{
                break
            }
            count += 1;
        }
        let disk = &self.start  + (rand_disk_x * self.blur) * &self.delta_x + (rand_disk_y * self.blur) * &self.delta_y;
        let target = &self.upper_left + (ux * &self.delta_x) + (uy * &self.delta_y);
        let dir = (&target-&self.start).normalize();
        Ray { start: disk, dir }
    }
}


impl Default for Camera {
    fn default() -> Self {
        Self::new(&Vec3::new(0., -5., 0.), &Vec3::default(), 90., &Vec3::new(0., 0., 1.), 0.)
    }
}