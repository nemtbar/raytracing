use crate::vec3::{Point, Vec3};
use crate::render::Pixel;
pub struct HitInfo {
    pub p: Point,
    pub normal: Vec3,
    pub color: Vec3,
}

pub enum Object {
    Sphere {pos: Vec3, col: Vec3, rad: f32},
    Plane {pos: Vec3, normal: Vec3, col: Vec3}
}

impl Object {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>{
        match self {
            //https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
            Self::Sphere {pos, col, rad} => {
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
                        Some(HitInfo{p: &ray.dir * t0, normal, color: col.clone()})
                    }
                }
            }
            Self::Plane {pos, normal, col} => {
                //https://www.cs.princeton.edu/courses/archive/fall00/cs426/lectures/raycast/sld017.htm
                //pos-vec dot normal = 0
                let mut n = normal.clone();
                if ray.dir.dot(normal) > 0. {
                    n = normal * -1.;
                }
                let mut t = pos.dot(&n) - &ray.start.dot(&n);
                t /= ray.dir.dot(&n);
                if t > 0. {
                    let p = &ray.start + &ray.dir * t;
                    return Some(HitInfo{p, normal: n, color: col.clone(), });
                }
                None
            }
        }
    }
    pub fn bounce(mut ray: Ray, objs: &Vec<Object>, max_bounce: u8) -> Pixel{
        let mut color: Vec3 = Vec3::new(1., 1., 1.);
        for i in 0..max_bounce {
            let inter = Self::hit_all(&ray, &objs);
            match inter {
                Some(hit) => {
                    let len = (&hit.p - &ray.start).length();
                    if len < 0.001 {
                        break;
                    }
                    if i != 0 {
                        color = &color * &(&hit.color * (len/4.).max(0.3).min(1.));
                    } else {
                        color = hit.color;
                    }
                    //lambertian reflection
                    let poi = &hit.p + &hit.normal;
                    ray.dir = (poi+Vec3::random() - &hit.p).normalize();
                    ray.start = hit.p;

                }
                _ => {
                    if i == 0 {
                        color = color.lerp(&Vec3::new(0.478, 0.859, 0.949), (ray.dir.z + 1.) / 2.)
                    }
                    break;
                }
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
