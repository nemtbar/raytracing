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
struct Ray {
    start: Point,
    dir: Vec3,
}

impl Ray {
    fn new(start: Point, dir: Vec3) -> Self{
        Self{start, dir}
    }
}

struct Sphere {
    pos: Vec3,
    col: Pixel,
    rad: f32,
}

impl Sphere {
    fn new(pos: Vec3, col: Pixel, rad: f32) -> Self{
        Self {pos, col, rad}
    }
}


fn intersect(ray: Ray, spheres: Vec<Sphere>) -> Option<Pixel> {
    //https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
    let mut min_d: f32 = 100000.;
    let mut color = None;
    for i in 0..spheres.len(){
        let sphere = &spheres[i];
        let l = &sphere.pos - &ray.start;
        let tc = l.dot(&ray.dir);
        if tc < 0.0{
            continue;
        } else {
            let d = (l.length() * l.length())-(tc*tc);
            let rad2 = sphere.rad * sphere.rad;
            if d > rad2|| d > min_d{
                continue;
            } else {
                let thc = (rad2 - d).sqrt();
                let t0 = tc - thc;
                let normal = (&ray.dir * t0 - &sphere.pos).normalize();
                min_d = d;
                color = Some(Pixel::new((normal.x * 255.) as u8, (normal.y * 255.) as u8, (normal.z * 255.) as u8));
            }
        }
    }
    color


}

fn frag(x: usize, y: usize) -> Pixel{
    //uv coordinates between -g->g
    let g = 1.;
    let ux = (x as f32) / (WIDTH as f32) * (g  * 2.)- g;
    let uy = ((y as f32) / (HEIGHT as f32) * (g * 2.) - g) * -1.;
    let camera: Point = Vec3::new(0., -5., 0.);
    let dir = (Vec3::new(0., 0., 0.)-&camera).normalize();
    
    let point: Point = &camera + &dir;
    let spheres = vec![
        Sphere::new(Vec3::new(0., 0., 0.), Pixel::new(255, 0, 0), 1.5),
    ];
    match intersect(Ray::new(camera, dir), spheres) {
        Some(color) => return color,
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