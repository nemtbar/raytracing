mod vec3;
use vec3::{Vec3, Point};
use std::fs::File;
use std::io::{self, Write};
const WIDTH: usize = 500;
const HEIGHT: usize = 500;

#[derive(Clone, Copy, Default)]
pub struct Pixel{
    r: u8,
    g: u8,
    b: u8
}

impl Pixel {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self{r, g, b}
    }
}


struct Image{
    pixels: [[Pixel; WIDTH]; HEIGHT]
}

impl Image{
    pub fn new() -> Self{
        Self{ pixels: [[Pixel::default(); WIDTH]; HEIGHT] }
    }

    pub fn new_with_method(method: impl Fn(usize, usize) -> Pixel) -> Self{
        let mut image = Self::new();
        for row in 0..HEIGHT{
            for col in 0..WIDTH{
                image.pixels[row][col] = method(col, row);
            }
        }
        image
    }
    fn display(&self) -> io::Result<()>{
        let mut file = File::create("sample.ppm")?;
        writeln!(file, "P3\n{} {}\n255", self.pixels.len(), self.pixels[0].len())?;
        for y in 0..self.pixels.len(){
            let first = self.pixels[y][0];
            let mut text = format!("{} {} {} ", first.r, first.g, first.b);
            for x in 1..self.pixels[0].len(){
                let p = self.pixels[y][x];
                text += format!("{} {} {} ", p.r, p.g, p.b).as_str();
            }
            write!(file, "{}", text)?;
        }
        Ok(())
    }
}
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
    p: Point,
    normal: Vec3,
    color: Pixel
}

pub enum Object {
    Sphere {pos: Vec3, col: Pixel, rad: f32}
}

impl Object {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>{
        //https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
        match self {
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
                        //let color = Pixel::new((normal.x * 255.) as u8, (normal.y * 255.) as u8, (normal.z * 255.) as u8);
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


fn frag(x: usize, y: usize) -> Pixel{
    //uv coordinates between -g->g
    let g = 1.;
    let ux = (x as f32) / (WIDTH as f32) * (g  * 2.)- g;
    let uy = ((y as f32) / (HEIGHT as f32) * (g * 2.) - g) * -1.;

    //define important points
    let mut camera: Point = Vec3::new(0., -5., 0.);
    let mut dir = Vec3::new(ux, camera.y+1., uy)-&camera;

    //rotation
    let angle = 90.;
    camera = camera.rot_x(angle);
    dir = dir.rot_x(angle).normalize();
    let ray = Ray::new(camera, dir);
    let objects: Vec<Object> = vec![
        Object::Sphere {pos: Vec3::new(0., 0., 0.), col: Pixel::new(255, 0, 0), rad: 1.}
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