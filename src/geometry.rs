use crate::vec3;
use crate::render;
use render::Pixel;
use vec3::{Point, Vec3};
pub struct HitInfo {
    pub p: Point,
    pub normal: Vec3,
    pub color: Vec3,
    pub emmision: f32
}

pub enum Object {
    Sphere {pos: Vec3, col: Vec3, rad: f32, emmision: f32}
}

impl Object {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>{
        match self {
            //https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
            Self::Sphere {pos, col, rad, emmision} => {
                let l = pos - &ray.start;
                let tc = l.dot(&ray.dir);
                if tc < 0.0{
                    None
                } else {
                    let d = l.length_squared()-tc*tc;
                    let rad2 = rad * rad;
                    if d > rad2{
                        None
                    } else {
                        let thc = (rad2 - d).sqrt();
                        let t0 = tc - thc;
                        let normal = (&ray.dir * t0 - pos) / rad;
                        Some(HitInfo{p: &ray.dir * t0, normal, color: col.clone(), emmision: emmision.clone()})
                    }
                }
            }
        }
    }
    pub fn bounce(mut ray: Ray, objs: &Vec<Object>, count: u8) -> Pixel{
        let mut light = Vec3::new(0., 0., 0.);
        let mut color = Vec3::new(1., 1., 1.);
        for _ in 0..count {
            let inter = Self::hit_all(&ray, &objs);
            match inter {
                Some(hit) => {
                    ray.start = hit.p;
                    ray.dir = Vec3::random(&hit.normal);
                    light = &light + &(&color * &hit.emmision) * &hit.color;
                    color = &color * &hit.color;

                }
                _ => break,
            }
        }
        Pixel::new((light.x * 255.) as u8, (light.y * 255.) as u8, (light.z * 255.) as u8)
    }
    
    pub fn hit_all(ray: &Ray, lis: &Vec<Self>) -> Option<HitInfo>{
        let mut inf: Option<HitInfo> = None;
        let mut min_dist = 100000.;
        for obj in lis {
            match Self::intersect(&obj, ray){
                Some(i) => {
                    let len = (&i.p - &ray.start).length();
                    if len < min_dist {
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