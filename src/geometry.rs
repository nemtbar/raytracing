use crate::vec3::{Point, Vec3};
use crate::render::Pixel;
pub struct HitInfo {
    pub p: Point,
    pub normal: Vec3,
    pub color: Vec3,
    pub reflection: Material
}

#[derive(Clone)]
pub enum Material {
    Diffuse (),
    Reflective ()
}
fn mix(col1: &Vec3, col2: &Vec3, mat: &Material, len: f32) -> Vec3 {
    match mat {
        Material::Diffuse() => {
            col1 * &(col2 * (len/2.).max(0.3).min(1.))
        }

        _ => col1.clone()
    }
}

fn scatter(ray: &Ray, hit: &HitInfo, mat: &Material) -> Ray {
    let mut sol: Ray = Ray::new(hit.p.clone(), Vec3::default());
    match mat {
        Material::Diffuse() => {
            //lambertian reflection
            let poi = &hit.p + &hit.normal;
            sol.dir = (poi+Vec3::random() - &hit.p).normalize();
            //ray.dir = hit.normal;

        }
        Material::Reflective() => {
            sol.dir = &hit.p + (&hit.normal - &ray.dir) * -2.;
        }
    }
    sol
}
pub enum Object {
    Sphere {pos: Vec3, col: Vec3, rad: f32, mat: Material},
    Plane {pos: Vec3, normal: Vec3, col: Vec3, mat: Material}
}

impl Object {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>{
        assert!(ray.dir.length() > 0.999 && ray.dir.length() < 1.001);
        match self {
            //https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
            Self::Sphere {pos, col, rad, mat} => {
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
                        let normal = (&ray.dir * inters - pos).normalize();
                        Some(HitInfo{p: &ray.start + &ray.dir * inters, normal, color: col.clone(), reflection: mat.clone()})
                    }
                }
            }
            Self::Plane {pos, normal, col, mat} => {
                //https://www.cs.princeton.edu/courses/archive/fall00/cs426/lectures/raycast/sld017.htm
                //pos-vec dot normal = 0
                let denom = normal.dot(&ray.dir);
                let t = (pos - &ray.start).dot(normal) / denom;
                if t > 0. {
                    let hit = HitInfo{p: &ray.start + &ray.dir * t, normal: normal.clone(), color: col.clone(), reflection: mat.clone()};
                    return Some(hit);
                }
                None
            }
        }
    }
    pub fn bounce(mut ray: Ray, objs: &Vec<Object>, max_bounce: u8) -> Pixel{
        let mut color: Vec3 = Vec3::new(1., 1., 1.);
        let mut last_mat = Material::Diffuse();
        for i in 0..max_bounce {
            let inter = Self::hit_all(&ray, &objs);
            match inter {
                Some(hit) => {
                    let len = (&hit.p - &ray.start).length();
                    if len < 0.001 {
                        break;
                    }
                    last_mat = hit.reflection.clone();
                    color = mix(&color, &hit.color, &hit.reflection, len);
                    ray = scatter(&ray, &hit, &hit.reflection);
                }
                _ => {
                    let sky = Vec3::new(1., 1., 1.).lerp(&Vec3::new(0.478, 0.859, 0.949), (ray.dir.z + 1.) / 2.);
                    if i == 0 {
                        color = sky;
                    } else {
                        color = mix(&sky, &color, &last_mat, 1000.);
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
