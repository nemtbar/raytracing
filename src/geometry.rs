use crate::vec3::{Point, Vec3};
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
    Glass()
}

#[derive(Clone, PartialEq)]
pub struct Material {
    pub color: Vec3,
    pub refl: Reflection
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
            sol.dir = (&hit.normal - &ray.dir * -1.) * 2.;
            sol.dir = sol.dir.normalize() + Vec3::random() * roughness;
            sol.dir = sol.dir.normalize();
        }
        Reflection::Glass() => {
            sol.dir = &hit.normal * -1.;
        }
    }
    sol
}

fn mix(a: Vec3, b: Vec3, mat: Reflection) -> Vec3 {
    match mat {
        Reflection::Glass() =>{
            b
        }

        _ => {
            a*(b*0.9)
        }
    }
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
                        let normal = (&ray.dir * inters - pos).normalize();
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
    pub fn bounce(ray: &Ray, objs: &Vec<Object>, max_bounce: u8) -> Vec3{
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
   
    
    pub fn hit_all(ray: &Ray, lis: &Vec<Self>) -> Option<HitInfo>{
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
    bases: Vec<Vec<f32>>
}

impl Camera {
    pub fn new(focus: Vec3, angle: Vec3, dist: f32) -> Self {
        let mut start = Vec3::new(0., -1., 0.).rot_x(angle.x).rot_y(angle.y).rot_z(angle.z);
        start = start * dist + &focus;
        let b_vec = [Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.), Vec3::new(0., 0., 1.)];
        for i in b_vec.clone() {
            i.rot_x(angle.x).rot_y(angle.y).rot_z(angle.z);
        }
        let bases = vec![vec![b_vec[0].x, b_vec[0].y, b_vec[0].z], 
                                    vec![b_vec[1].x, b_vec[1].y, b_vec[1].z], 
                                    vec![b_vec[2].x, b_vec[2].y, b_vec[2].z]];
        Self { start, bases }
    }

    pub fn shoot(&self, ux: f32, uy: f32) -> Ray {
        let dir = (Vec3::new(ux, 1., uy) * self.bases.clone()).normalize();
        Ray { start: self.start.clone(), dir }
    }
}
