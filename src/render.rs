use crate::{Uniforms, WIDTH, HEIGHT, Vec3};
use rayon::prelude::*;
use indicatif::ProgressBar;
use image::RgbImage;
use std::sync::Arc;
use std::ops::{Deref, Index};

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

pub fn display<F>(func: F, input: Uniforms) -> Picture
where F: Fn(usize, usize, &Uniforms) -> Pixel + Sync + Send {
    let mut buffer = Picture::empty(WIDTH as u32, HEIGHT as u32);
    println!("Rendering image...");
    let pb = ProgressBar::new(HEIGHT as u64);
    for y in 0..HEIGHT {
        let row = parallel_row(&func, &input, y);
        for (x, pix) in row.iter().enumerate() {
            buffer.set_pixel(x as u32, y as u32, *pix);
        }
        pb.inc(1);
    }
    pb.finish_and_clear();
    println!("done!");
    buffer
}


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

    pub fn from_vec(vec: Vec3) ->Self{
        let r = (vec.x*255.) as u8;
        let g = (vec.y*255.) as u8;
        let b = (vec.z*255.) as u8;
        Self {r, g, b}
    }

    pub fn to_vec(&self)->Vec3{
        Vec3::new(self.r as f32/255., self.r as f32/255., self.r as f32/255.)
    }
}

#[derive(Clone, Debug)]
enum Vecu8{
    Inmut(Arc<Vec<u8>>),
    Mutbl(Vec<u8>)
}

impl Vecu8{
    pub fn set(&mut self, index:usize, value: u8){
        match self {
            Self::Mutbl(vec) => {vec[index] = value},
            _ => {}
        }
    }
}

impl Deref for Vecu8{
    type Target = Vec<u8>;
    
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Mutbl(vec) => vec,
            Self::Inmut(vec) => vec
        }
    }
}

impl Index<usize> for Vecu8{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Inmut(vec) => &vec[index],
            Self::Mutbl(vec) => &vec[index]
        }
    }
}



#[derive(Clone, Debug)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    data: Vecu8,
}

impl Picture {
    pub fn new(img: RgbImage) -> Self {
        let (width, height) = img.dimensions();
        Self { width, height, data: Vecu8::Inmut(Arc::new(img.into_vec()))}
    }

    pub fn empty(width: u32, height: u32) ->Self {
        Self { width, height, data: Vecu8::Mutbl(vec![0_u8; (width*height*3) as usize]) }
    }

    pub fn get_first_index(&self, x:u32, y:u32) -> usize{
        let y_index = y*(self.width*3);
        (x*3+y_index) as usize
    }

    pub fn get_pixel_normalized(&self, pos: (u32, u32)) -> Vec3{
        self.get_pixel(pos).to_vec()

    }

    pub fn get_pixel(&self, pos: (u32, u32)) ->Pixel{
        assert!(pos.0 < self.width && pos.1 < self.height, "x:{} or y:{}: is out of bounds", pos.0, pos.1);
        let index = self.get_first_index(pos.0, pos.1);
        Pixel::new(self.data[index], self.data[index+1], self.data[index+2])
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Pixel){
        let index = self.get_first_index(x, y);
        self.data.set(index, color.r);
        self.data.set(index+1, color.g);
        self.data.set(index+2, color.b);
    }

    fn get_kernel(&self, pos: (i32, i32), step: i32) -> Vec<Pixel> {
        let mut pixies: Vec<Pixel> = vec![];
        for y in -step..step {
            for x in -step..step {
                let crr: (u32, u32) = ((pos.0+x).max(0).min((self.width-1) as i32) as u32, (pos.1+y).max(0).min((self.height-1) as i32) as u32);
                let index = self.get_first_index(crr.0, crr.1);
                pixies.push(Pixel::new(self.data[index], self.data[index+1], self.data[index+2]));
            }
        }

        pixies
    }

    fn avg_color(&self, pos: (i32, i32), step: i32)->Pixel{
        todo!()
    }

    fn variance(&self, pos: (i32, i32), step: i32)->f32{
        let mut pixies: Vec<[u8; 3]> = vec![];
        for y in -step..step {
            for x in -step..step {
                let crr: (u32, u32) = ((pos.0+x).max(0).min((self.width-1) as i32) as u32, (pos.1+y).max(0).min((self.height-1) as i32) as u32);
                let index = self.get_first_index(crr.0, crr.1);
                pixies.push([self.data[index], self.data[index+1], self.data[index+2]]);
            }
        }

        0.
    }

    pub fn blur(&self, step: u32) -> Self {
        todo!()
    }

    pub fn to_buffer(&self)->RgbImage{
        let buf = (*self.data).clone();
        RgbImage::from_vec(self.width, self.height, buf).unwrap()
    }
}
