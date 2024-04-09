use crate::vec3::{Point, Vec3};
use crate::render::Pixel;
pub struct HitInfo {
    pub p: Point,
    pub normal: Vec3,
    pub color: Vec3,
    pub emmision: f32
}

pub enum Object {
    Sphere {pos: Vec3, col: Vec3, rad: f32, emmision: f32},
    Plane {pos: Vec3, normal: Vec3, col: Vec3, emmision: f32}
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
                        let normal = (&ray.dir * t0 - pos).normalize();
                        Some(HitInfo{p: &ray.dir * t0, normal, color: col.clone(), emmision: emmision.clone()})
                    }
                }
            }
            Self::Plane {pos, normal, col, emmision} => {
                //https://www.cs.princeton.edu/courses/archive/fall00/cs426/lectures/raycast/sld017.htm
                //pos-vec dot normal = 0
                let mut t = pos.dot(normal) - &ray.start.dot(normal);
                t /= ray.dir.dot(normal);
                if t > 0. {
                    let p = &ray.start + &ray.dir * t;
                    return Some(HitInfo{p, normal: normal.clone(), color: col.clone(), emmision: emmision.clone()});
                }
                None
            }
        }
    }
    pub fn bounce(mut ray: Ray, objs: &Vec<Object>, max_bounce: u8) -> Pixel{
        let mut color: Vec3 = Vec3::new(1., 1., 1.);
        for _ in 0..max_bounce {
            let inter = Self::hit_all(&ray, &objs);
            match inter {
                Some(hit) => {
                    color = &color * &hit.color * (0.01/(&hit.p - &ray.start).length()).max(0.2).min(1.);
                    ray.start = hit.p;
                    ray.dir = Vec3::random(&hit.normal);

                }
                _ => break
            }
        }
        Pixel::new((color.x * 255.) as u8, (color.y * 255.) as u8, (color.z * 255.) as u8)
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
