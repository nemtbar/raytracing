extern crate image;
use image::{RgbImage, Rgb};


const WIDTH: u32 = 255; 
const HEIGHT: u32 = 255; 

fn map(value: f32, min1: f32, max1: f32, min2: f32, max2: f32)-> f32 {
    return min2 + (value - min1) * (max2 - min2) / (max1 - min1);
}

#[derive(Clone, Copy, Debug)]
struct Vec3{
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    fn new(a: f32, b: f32, c: f32) -> Vec3{
        return Vec3{x: a, y: b, z: c};
    }

    fn add(mut self, other: Vec3)-> Vec3{
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        return self;
    }

    fn sub(mut self, other: Vec3)-> Vec3{
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        return self;
    }

    fn normalize(&mut self){
        self.x /= self.length();
        self.y /= self.length();
        self.z /= self.length();
    }

    fn length(self) -> f32 {
        return f32::sqrt(f32::powf(self.x, 2.0)+
        f32::powf(self.y, 2.0)+f32::powf(self.z, 2.0))
    }

    fn mult(mut self, a: f32) -> Vec3{
        self.x *= a;
        self.y *= a;
        self.z *= a;
        return self;
    }

}

fn shoot(start: Vec3, dir: Vec3)-> f32{
    let sphere = Vec3::new(0.0, 0.0, 0.0);
    let mut traveled = 0.0;
    for _i in 0..100{
        let ray = start.clone().add(dir.clone().mult(traveled));
        let d = ray.sub(sphere).length()-1.0;
        if d < 0.001{break;}
        if d > 100.0{break;}
        traveled += d;
    }
    return map(traveled, 0.0, 100.0, 255.0, 0.0);
}

fn main(){
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    let camera = Vec3::new(0.0, -3.0, 0.0);
    for x in 0..WIDTH{
        for y in 0..HEIGHT {
            let uv_x = map(x as f32, 0.0, WIDTH as f32, 0.0, 1.0);
            let uv_y = map(y as f32, 0.0, HEIGHT as f32, 0.0, 1.0);
            let mut dir = Vec3::new(uv_x*2.0-1.0, camera.y+1.0, uv_y*2.0-1.0);
            dir = dir.sub(camera.clone());
            dir.normalize();
            let dis = shoot(camera, dir) as u8;
            img.put_pixel(x, y, Rgb([dis, dis, dis]));
        }
    }
    img.save("output.png").unwrap();
}