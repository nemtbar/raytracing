mod vec3;
mod render;
use vec3::{Vec3, Point};
use render::{Pixel, Image};

//https://raytracing.github.io/books/RayTracingInOneWeekend.html

#[derive(Debug)]
pub struct Ray {
    pub start: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(start: Point, dir: Vec3) -> Self{
        Self{start, dir}
    }
}

pub struct HitInfo {
    pub p: Point,
    pub normal: Vec3,
    pub color: Pixel
}

pub enum Object {
    Sphere {pos: Vec3, col: Pixel, rad: f32}
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
                    let d = (l.length() * l.length())-(tc*tc);
                    let rad2 = rad * rad;
                    if d > rad2{
                        None
                    } else {
                        let thc = (rad2 - d).sqrt();
                        let t0 = tc - thc;
                        let normal = (&ray.dir * t0 - pos) / rad;
                        Some(HitInfo{p: &ray.dir * t0, normal, color: *col})
                    }
                }
            }
        }
    }


    pub fn hit_all(ray: &Ray, lis: Vec<Self>) -> Option<HitInfo>{
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

//fragment shader -> runs for every pixel
fn frag(x: usize, y: usize, width: usize, height: usize) -> Pixel{
    //uv coordinates between -g->g
    let g = 1.;
    let ux = (x as f32) / (width as f32) * (g  * 2.)- g;
    let uy = ((y as f32) / (height as f32) * (g * 2.) - g) * -1.;

    //define important points
    let camera: Point = Vec3::new(0., -5., 0.);
    let dir = (Vec3::new(ux, camera.y+1., uy)-&camera).normalize();

    let ray = Ray::new(camera, dir);
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(0., 0., 0.), col: Pixel::new(255, 0, 0), rad: 1.},
        Object::Sphere { pos: Vec3::new(0., 0., -40.), col: Pixel::new(70, 235, 73), rad: 39.}
    ];
    match Object::hit_all(&ray, objects) {
        Some(i) => i.color,
        None => {
            //background color
            let value =(uy + g) / (2.* g);
            let col = Vec3::new(255., 255., 255.).lerp(&Vec3::new(81., 187., 232.), value);
            Pixel::new(col.x as u8, col.y as u8, col.z as u8)
        }
    }
    
}

fn main(){
    let sample = Image::new_with_method(frag);
    sample.display().unwrap(); 
}