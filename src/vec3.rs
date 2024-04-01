use std::ops::{Add, Mul, Sub, Div};

#[derive(Default, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Vec3{x, y, z}
    }

    pub fn length(&self) -> f32{
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot()
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

