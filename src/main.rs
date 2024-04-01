mod vec3;
use vec3::Vec3;
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

fn simple(x: usize, y: usize) -> Pixel{
    let ux = (x as f32) / (WIDTH as f32);
    let uy = 1.0-(y as f32) / (HEIGHT as f32);
    let mut pix = Pixel::default();
    pix.r = (ux * 255.0) as u8;
    pix.g = (uy * 255.0) as u8;
    pix.b = 255;
    pix
    
}

fn main(){
    let v1 = Vec3::new(0., 1., 2.);
    let v2 = Vec3::new(10., 4., 1.);
    let v12 = &v1 * &2.0;
    println!("{:?}", v12);
    let sample = Image::new_with_method(WIDTH, HEIGHT, simple);
    sample.display().unwrap(); 
}