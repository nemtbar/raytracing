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
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self{r, g, b}
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