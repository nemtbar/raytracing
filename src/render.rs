use std::fs::File;
use std::io::{self, Write};
use crate::vec3;
use vec3::Vec3;
const WIDTH: usize = 500;
const HEIGHT: usize = 500;


#[derive(Clone, Copy, Default)]
pub struct Pixel{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self{r, g, b}
    }

    pub fn as_vector(pix: &Self) -> Vec3{
        Vec3::new(pix.r as f32, pix.g as f32, pix.b as f32)
    }

    pub fn lerp(&self, other: &Self, value: f32) -> Self {
        let v = Self::as_vector(self).lerp(&Self::as_vector(other), value);
        Self::new(v.x as u8, v.y as u8, v.z as u8)
    }
}


pub struct Image{
    pixels: [[Pixel; WIDTH]; HEIGHT]
}

impl Image{
    pub fn new() -> Self{
        Self{ pixels: [[Pixel::default(); WIDTH]; HEIGHT] }
    }

    pub fn new_with_method(method: impl Fn(usize, usize, usize, usize) -> Pixel) -> Self{
        let mut image = Self::new();
        for row in 0..HEIGHT{
            for col in 0..WIDTH{
                image.pixels[row][col] = method(col, row, WIDTH, HEIGHT);
            }
        }
        image
    }
    pub fn display(&self) -> io::Result<()>{
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