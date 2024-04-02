use std::ops::{Add, Mul, Sub, Div};

#[derive(Default, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub type Point = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Vec3{x, y, z}
    }

    pub fn length(&self) -> f32{
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn normalize(&self) -> Vec3 {
        self / self.length()
    }
    pub fn lerp(&self, other: &Self, value: f32) -> Self {
        assert_eq!(value >= 0.0 && value <= 1.0, true);
        self*(1.0-value) + other*value
    }
    pub fn cross(&self, other: &Self) -> Self{
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
}
// add
impl Add<&Vec3> for &Vec3{
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z)
    }
}

impl Add<Vec3> for &Vec3{
    type Output = Vec3;
    
    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl Add<&Vec3> for Vec3{
    type Output = Vec3;
    
    fn add(self, rhs: &Vec3) -> Self::Output {
        &self + rhs
    }
}
impl Add for Vec3{
    type Output = Vec3;
    
    fn add(self, rhs: Vec3) -> Self::Output {
        &self + &rhs
    }
}

//sub
impl Sub<&Vec3> for &Vec3{
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z)
    }
}

impl Sub<Vec3> for &Vec3{
    type Output = Vec3;
    
    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl Sub<&Vec3> for Vec3{
    type Output = Vec3;
    
    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}
impl Sub for Vec3{
    type Output = Vec3;
    
    fn sub(self, rhs: Vec3) -> Self::Output {
        &self - &rhs
    }
}


//mult
impl Mul<&f32> for &Vec3{
    type Output = Vec3;
    fn mul(self, rhs: &f32) -> Self::Output {
        Vec3::new(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}

impl Mul<f32> for &Vec3{
    type Output = Vec3;
    
    fn mul(self, rhs: f32) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&f32> for Vec3{
    type Output = Vec3;
    
    fn mul(self, rhs: &f32) -> Self::Output {
        &self * rhs
    }
}
impl Mul<f32> for Vec3{
    type Output = Vec3;
    
    fn mul(self, rhs: f32) -> Self::Output {
        &self * &rhs
    }
}

//div
impl Div<&f32> for &Vec3{
    type Output = Vec3;
    fn div(self, rhs: &f32) -> Self::Output {
        Vec3::new(self.x/rhs, self.y/rhs, self.z/rhs)
    }
}

impl Div<f32> for &Vec3{
    type Output = Vec3;
    
    fn div(self, rhs: f32) -> Self::Output {
        self / &rhs
    }
}

impl Div<&f32> for Vec3{
    type Output = Vec3;
    
    fn div(self, rhs: &f32) -> Self::Output {
        &self / rhs
    }
}
impl Div<f32> for Vec3{
    type Output = Vec3;
    
    fn div(self, rhs: f32) -> Self::Output {
        &self / &rhs
    }
}
