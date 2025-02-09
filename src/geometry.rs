
use crate::{vec3::{Point, Vec3}, WIDTH, HEIGHT};
use rand::Rng;
pub struct HitInfo {
    pub p: Point,
    pub normal: Vec3,
    pub material: Material
}

#[derive(Clone, PartialEq)]
pub enum Reflection {
    Diffuse(),
    //roughness is normalized
    Metal{roughness: f32},
    Glass{reflective: f32}
}

#[derive(Clone, PartialEq)]
pub struct Material {
    pub color: Vec3,
    pub refl: Reflection
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
    let refl = reflectance(cos_theta.abs().min(1.), eta) > rand::thread_rng().gen_range(0.0..1.0);
    //sin^2theta + cos^2theta = 1
    // total internal reflection
    if (eta * (1.- cos_theta * cos_theta).abs().sqrt()) > 1. || refl{
        return (incoming + normal * 2.).normalize()
    }
    let r_out_perp: Vec3 = eta * &(incoming + normal * cos_theta);
    let r_out_parallel: Vec3 = (1.-r_out_perp.length_squared()).abs().sqrt() * -1. * normal;
    return r_out_parallel + r_out_perp;
}

fn scatter(ray: &Ray, hit: &HitInfo) -> Ray {
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
            assert!(sol.dir.length() > 0.999 && sol.dir.length() < 1.001, "scatter glass");
        }

    }
    sol
}


#[derive(Clone)]
pub enum Object {
    Sphere {pos: Vec3, rad: f32, mat: Material},
    Plane {pos: Vec3, normal: Vec3, mat: Material}
}

impl Object {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>{
        assert!(ray.dir.length() > 0.999 && ray.dir.length() < 1.001);
        match self {
            //https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
            Self::Sphere {pos, rad, mat} => {
                let camera_self = pos - &ray.start;
                let project_len = camera_self.dot(&ray.dir);
                if project_len < 0.0{
                    None
                } else {
                    let closest = camera_self.length_squared()-project_len*project_len;
                    let rad2 = rad * rad;
                    if closest > rad2{
                        None
                    } else {
                        let t1c = (rad2 - closest).sqrt();
                        let inters = project_len - t1c;
                        if inters <= 0. {
                            return None;
                        }
                        let normal = (&ray.start + &ray.dir * inters - pos).normalize();
                        Some(HitInfo{p: &ray.start + &ray.dir * inters, normal, material: mat.clone()})
                    }
                }
            }
            Self::Plane {pos, normal, mat} => {
                //https://www.cs.princeton.edu/courses/archive/fall00/cs426/lectures/raycast/sld017.htm
                //pos-vec dot normal = 0
                let mut n = normal.clone();
                if n.dot(&ray.dir) > 0. {
                    n = n * -1.;
                }
                let denom = n.dot(&ray.dir);
                let t = (pos - &ray.start).dot(&n) / denom;
                if t > 0. {
                    let hit = HitInfo{p: &ray.start + &ray.dir * t, normal: n.clone(), material: mat.clone()};
                    return Some(hit);
                }
                None
            }
        }
    }
    pub fn bounce(ray: &Ray, objs: &[Object], max_bounce: u8) -> Vec3{
        assert!(ray.dir.length() > 0.999 && ray.dir.length() < 1.001);
        if max_bounce <= 0 {
            return Vec3::new(0., 0., 0.);
        }

        match Self::hit_all(ray, objs) {
            Some(hit) => {
                let r = scatter(ray, &hit);
                let future = Self::bounce(&r, objs, max_bounce - 1) * 0.9;
                return &hit.material.color * &future;
            }

            None => {
                
                let value = (ray.dir.z + 1.) / 2.;
                return Vec3::new(1., 1., 1.).lerp(&Vec3::new(0.5, 0.7, 1.), value);
            }
        } 
    }
   
    
    pub fn hit_all(ray: &Ray, lis: &[Self]) -> Option<HitInfo>{
        let mut inf: Option<HitInfo> = None;
        let mut min_dist = 100000.;
        for obj in lis {
            match Self::intersect(&obj, ray){
                Some(i) => {
                    let len = (&i.p - &ray.start).length();
                    if len < 0.0001{
                        continue;
                    }
                    else if len < min_dist {
                        inf = Some(i);
                        min_dist = len;
                    } else {
                        continue;
                    }
                }

                None => continue,
            }
        }

        inf
    }
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
}

impl Camera {
    pub fn new(lookfrom: &Point, lookat: &Point, vertical_fov: f32, up: &Vec3) -> Self {
    let focal_length = (lookfrom - lookat).length();
    let theta = vertical_fov.to_radians();
    let h = (theta/2.).tan();
    let viewport_height = 2. * h * focal_length;
    let viewport_width = WIDTH as f32 / HEIGHT as f32 * viewport_height;
    let w = (lookfrom - lookat).normalize();
    let u = up.cross(&w);
    let v = w.cross(&u);
    let viewport_u = u * viewport_width;
    let viewport_v = -1. * &v * viewport_height;
    let pixel_delta_u = &viewport_u / WIDTH as f32;
    let pixel_delta_v = &viewport_v / HEIGHT as f32;

    //let upper_left: Vec3 = lookfrom - (focal_length * &w) - (viewport_u / 2.) + (viewport_v / 2.);
    let upper_left = Vec3::new(0., 0., 0.) - (viewport_u / 2.) - (viewport_v / 2.);
    let pixel00 = upper_left + 0.5 * &(&pixel_delta_u + &pixel_delta_v);
    Self { start: lookfrom.clone(), upper_left: pixel00, delta_x: pixel_delta_u, delta_y: pixel_delta_v }
    }

    pub fn shoot(&self, ux: f32, uy: f32) -> Ray {
        let target = &self.upper_left + (ux * &self.delta_x) + (uy * &self.delta_y);
        let dir = (&target-&self.start).normalize();
        Ray { start: self.start.clone(), dir }
    }
}
