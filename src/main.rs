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


struct Image{
    pixels: [[Pixel; WIDTH]; HEIGHT]
}

impl Image{
    pub fn new() -> Self{
        Self{ pixels: [[Pixel::default(); WIDTH]; HEIGHT] }
    }

    pub fn new_with_method(height: usize, width: usize, method: impl Fn(usize, usize) -> Pixel) -> Self{
        let mut image = Self::new();
        for row in 0..height{
            for col in 0..width{
                image.pixels[row][col] = method(col, row);
            }
        }
        image
    }
    fn display(self) -> io::Result<()>{
        let mut file = File::create("sample.ppm")?;
        writeln!(file, "P3\n{} {}\n255", self.pixels.len(), self.pixels[0].len())?;
        for y in 0..self.pixels.len(){
            for x in 0..self.pixels[0].len(){
                let p = self.pixels[y][x];
                write!(file, "{} {} {} ", p.r, p.g, p.b)?;
            }
        }
        Ok(())
    }
}

struct Ray {
    start: Point,
    dir: Vec3,
}

impl Ray {
    fn new(start: Point, dir: Vec3) -> Self{
        Self{start, dir}
    }
}

fn intersect(ray: Ray, sphere: Point) -> bool {
    //https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
    let vec = &sphere - &ray.start;
    let len = vec.dot(&ray.dir);
    if len < 0.0 {
        false
    } else {
        let d = ((len * len) - (vec.length() * vec.length())).sqrt();
        if d > 1.0 {
            false
        } else {
            true
        }
    }
}

fn frag(x: usize, y: usize) -> Pixel{
    let ux = ((x as f32) / (WIDTH as f32) - 1.0) / 2.;
    let uy = ((y as f32) / (HEIGHT as f32) - 1.) / 2.;
    let camera: Point = Vec3::new(0., -2., 0.);
    let dir = Vec3::new(ux, 0., uy)-&camera.normalize();
    let f = intersect(Ray::new(camera, dir), Vec3::new(0., 0., 0.));
    match f {
        true => Pixel{r: 255, g: 255, b: 255},
        false => Pixel::default(),
    }
    
}

fn main(){
    let sample = Image::new_with_method(WIDTH, HEIGHT, frag);
    sample.display().unwrap(); 
}