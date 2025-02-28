use image::{RgbImage, ImageBuffer, Rgb};
use crate::{WIDTH, HEIGHT, Uniforms};
use rayon::prelude::*;
use indicatif::ProgressBar;
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
}

fn parallel_row<F>(func: F, input: &Uniforms, y: usize) -> [Pixel; WIDTH]
where
    F: Fn(usize, usize, &Uniforms) -> Pixel + Sync + Send,
{
    let mut arr: [Pixel; WIDTH] = [Pixel::default(); WIDTH];
    arr.par_iter_mut().enumerate().for_each(|(x, pixel)|
        *pixel = func(x, y, input)
    );
    arr
}

pub fn display<F>(func: F, input: Uniforms, name: &str)
where F: Fn(usize, usize, &Uniforms) -> Pixel + Sync + Send {
    let mut buffer: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    println!("Rendering image...");
    let pb = ProgressBar::new(HEIGHT as u64);
    for y in 0..HEIGHT {
        let row = parallel_row(&func, &input, y);
        for (x, pix) in row.iter().enumerate() {
            buffer.put_pixel(x as u32, y as u32, Rgb([pix.r, pix.g, pix.b]));
        }
        pb.inc(1);
    }
    pb.finish_and_clear();
    println!("done!");
    buffer.save(format!("{}.png", name)).unwrap();
}

pub fn transform(r: f32, g: f32, b: f32) -> Pixel {
    let r = r.max(0.).min(1.);
    let g = g.max(0.).min(1.);
    let b =b.max(0.).min(1.);
    Pixel::new((r * 255.) as u8, (g* 255.) as u8, (b* 255.) as u8)
}