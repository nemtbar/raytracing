use crate::{vec3::{Point, Vec3}, WIDTH, HEIGHT, textures::Texture};
use rand::Rng;
pub struct HitInfo {
    pub p: Point,
    pub normal: Vec3,
    pub material: Material,
    pub u: f32,
    pub v: f32
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
    pub tex: Texture
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

fn overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
    let over_min = min1.max(min2);
    let over_max = max1.min(max2);
    over_min < over_max
}


pub enum Object {
    Sphere {pos: Vec3, rad: f32, mat: Material},
    Plane {pos: Vec3, normal: Vec3, plane_type: Planes, mat: Material},
    BoundBox {min: Vec3, max: Vec3, inside: Vec<Object>},
}

pub enum Planes{
    Rect{delta_x: Vec3, delta_y: Vec3},
    Triangle{delta_x: Vec3, delta_y: Vec3},
    Disk{delta_x: Vec3, delta_y: Vec3, r: f32},
    Plane()
}

impl Planes{
    fn get_fn(&self)->Box<dyn Fn(f32, f32)->bool>{
        match self {
            Self::Triangle{delta_x: _, delta_y: _} => Box::new(|a: f32, b: f32| a > 0. && b > 0. && a+b < 1.),
            Self::Rect{delta_x: _, delta_y: _} => Box::new(|a: f32, b: f32| 0. < a && a < 1. && 0. < b && b < 1.),
            Self::Disk{delta_x: _, delta_y: _, r} =>  {
                let r = *r;
                Box::new(move |a: f32, b: f32| a*a+b*b < r)
            }
            _ => Box::new(|_, _| true)
        }
    }
}

impl Object {

    fn calc_quadrilet(p: &Vec3, u: &Vec3, v: &Vec3, w: &Vec3) -> (f32, f32){
        let alpha = w.dot(&p.cross(v));
        let beta = w.dot(&u.cross(&p));
        (alpha, beta)
    }

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
                        let hitp = &ray.start + &ray.dir * inters;
                        let (u,v) = Texture::sphere_uv_coord(pos, &hitp);
                        Some(HitInfo{p: hitp, normal, material: mat.clone(), u, v})
                    }
                }
            }
            Self::Plane {pos, normal, mat, plane_type} => {
                //https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html
                //pos-vec dot normal = 0
                let mut n = normal.clone();
                if n.dot(&ray.dir) > 0. {
                    n = n * -1.;
                }
                let denom = n.dot(&ray.dir);
                let t = (pos - &ray.start).dot(&n) / denom;
                if t > 0.{
                    let hit = HitInfo{p: &ray.start + &ray.dir * t, normal: n.clone(), material: mat.clone(), u: 0., v: 0.};
                    match plane_type {
                        Planes::Plane() => return Some(hit),
                        Planes::Disk {delta_x, delta_y, r:_ } => {
                            let z = delta_x.cross(delta_y);
                            let w = &z / &z.dot(&z);
                            let (alpha, beta) = Self::calc_quadrilet(&(&hit.p-pos), delta_x, delta_y, &w);
                            if plane_type.get_fn()(alpha, beta){
                                return Some(hit);
                            }
                            return None;
                        }   
                        Planes::Rect {delta_x, delta_y } => {
                            let n = delta_x.cross(delta_y);
                            let w = &n / &n.dot(&n);
                            let (alpha, beta) = Self::calc_quadrilet(&(&hit.p-pos), delta_x, delta_y, &w);
                            if plane_type.get_fn()(alpha, beta){
                                return Some(hit);
                            }
                            return None;
                        }
                        Planes::Triangle {delta_x, delta_y } => {
                            let n = delta_x.cross(delta_y);
                            let w = &n / &n.dot(&n);
                            let (alpha, beta) = Self::calc_quadrilet(&(&hit.p-pos), delta_x, delta_y, &w);
                            if plane_type.get_fn()(alpha, beta){
                                return Some(hit);
                            }
                            return None;
                        }
                    }
                }
                None  
            }
            Self::BoundBox { min, max, inside } => {
                let tx0 = ((min.x - ray.start.x)/ray.dir.x).min((max.x - ray.start.x)/ray.dir.x);
                let tx1 = ((min.x - ray.start.x)/ray.dir.x).max((max.x - ray.start.x)/ray.dir.x);
                let ty0 = ((min.y - ray.start.y)/ray.dir.y).min((max.y - ray.start.y)/ray.dir.y);
                let ty1 = ((min.y - ray.start.y)/ray.dir.y).max((max.y - ray.start.y)/ray.dir.y);
                let tz0 = ((min.z - ray.start.z)/ray.dir.z).min((max.z - ray.start.z)/ray.dir.z);
                let tz1 = ((min.z - ray.start.z)/ray.dir.z).max((max.z - ray.start.z)/ray.dir.z);
                let before_cam = tx1 > 0. && ty1 > 0. && tz1 > 0.;
                let nan = tx0.is_nan() || tx1.is_nan() || ty0.is_nan() || ty1.is_nan() || tz0.is_nan() || tz1.is_nan();
                let over =  overlap(tx0, tx1, ty0, ty1) && overlap(tx0, tx1, tz0, tz1) && overlap(tz0, tz1, ty0, ty1);
                if (nan || over) && before_cam {
                    return Self::hit_all(ray, inside);
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
                match hit.material.tex {
                    Texture::Solid { color } => return color * future,
                    Texture::Checker{color1, color2, size} => {
                        let x = (hit.p.x / size).round() as i32;
                        let y = (hit.p.y / size).round() as i32;
                        let z = (hit.p.z / size).round() as i32;

                        if (x+y+z)%2 == 0 {
                            return color1 * future
                        } else {
                            return color2 * future;
                        }
                    }
                    Texture::Img{img} => {
                        assert!(hit.u >= 0. && hit.u <= 1. && hit.v >= 0. && hit.v <= 1., "texture coord not in 0-1");
                        let x = (hit.u * (img.width as f32)) as usize;
                        let y = (hit.v * (img.height as f32)) as usize;
                        let color = img.get_pixel(x, y);
                        color * future
                    }
                }

            }

            None => {
                
                let value = (ray.dir.z + 1.) / 2.;
                return Vec3::new(1., 1., 1.).lerp(&Vec3::new(0.5, 0.7, 1.), value);
            }
        } 
    }
   
    
    pub fn hit_all(ray: &Ray, lis: &Vec<Object>) -> Option<HitInfo>{
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
    blur: f32
}


impl Camera {
    pub fn new(lookfrom: &Point, lookat: &Point, vertical_fov: f32, up: &Vec3, blur: f32) -> Self {
    assert!(up.length() > 0.999 && up.length() < 1.001, "up vector must be normalized at Camera::new");
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

    //let upper_left: Vec3 = lookfrom - (focal_length * &w) - (viewport_u / 2.) + (viewport_v / 2.);
    let upper_left = Vec3::new(0., 0., 0.) - (viewport_u / 2.) - (viewport_v / 2.);
    let pixel00 = upper_left + 0.5 * &(&pixel_delta_u + &pixel_delta_v);
    Self { start: lookfrom.clone(), upper_left: pixel00, delta_x: pixel_delta_u, delta_y: pixel_delta_v, blur }
    }

    pub fn shoot(&self, ux: f32, uy: f32) -> Ray {
        let mut count = 0;
        let mut rand_disk_x: f32;
        let mut rand_disk_y: f32;
        loop {
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