use std::ops::{Add, Div, Mul, Sub};
use rand::Rng;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub type Point = Vec3;
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self{
        Vec3{x, y, z}
    }
    pub const fn up()->Self{
        Self{x: 0., y: 0., z: 1.}
    }

    pub const fn side()->Self{
        Self{x: 1., y: 0., z: 0.}
    }

    pub const fn back()->Self{
        Self{x: 0., y: 1., z: 0.}
    }
    pub const fn new1(x: f32) -> Self{
        Vec3{x, y: x, z: x}
    }

    pub fn length(&self) -> f32{
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Self) -> Self {
        let sol = Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        ); 
        sol
    }
    pub fn normalize(&self) -> Vec3 {
        self / self.length()
    }
    pub fn lerp(&self, other: &Self, value: f32) -> Self {
        assert!(value >= 0.0 && value <= 1.0);
        self*(1.0-value) + other*value
    }
    pub fn mat_mult(&self, mat: &Vec<Vec<f32>>) -> Self{
        Self::new(
            self.x * mat[0][0] + self.y * mat[0][1] + self.z * mat[0][2],
            self.x * mat[1][0] + self.y * mat[1][1] + self.z * mat[1][2],
            self.x * mat[2][0] + self.y * mat[2][1] + self.z * mat[2][2]
        )
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut c: u8 = 0;
        let mut sol = Self::new(0., 0., 0.);
        while c < 50 {
            let x = rng.gen_range(-1.0..=1.0);
            let y = rng.gen_range(-1.0..=1.0);
            let z = rng.gen_range(-1.0..=1.0);
            sol = Self {x, y, z};
            if sol.length_squared() <= 1. {
                sol = sol.normalize();
                break;
            }
            c += 1;
        }
        sol
    }

    pub fn rot_z(&self, degree: f32) -> Self{
        let theta = f32::to_radians(degree);
        let rot = vec![
            vec![theta.cos(), -(theta.sin()), 0.],
            vec![theta.sin(), theta.cos(), 0.],
            vec![0., 0., 1.]
        ];
        self.mat_mult(&rot)
    }

    pub fn rot_y(&self, degree: f32) -> Self {
        let theta = f32::to_radians(degree);
        let rot = vec![
            vec![theta.cos(), 0., theta.sin()],
            vec![0., 1., 0.],
            vec![-(theta.sin()), 0., theta.cos()]
        ];
        self.mat_mult(&rot)
    }

    pub fn rot_x(&self, degree: f32) -> Self {
        let theta = f32::to_radians(degree);
        let rot = vec![
            vec![1., 0., 0.],
            vec![0., theta.cos(), -(theta.sin())],
            vec![0., theta.sin(), theta.cos()]
        ];
        self.mat_mult(&rot)
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

impl Add<f32> for &Vec3{
    type Output = Vec3;
    
    fn add(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x+rhs, self.y+rhs, self.z+rhs)
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

impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}
impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        &self * rhs
    }
}
impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}
//matrix multiplication
impl Mul<Vec<Vec<f32>>> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec<Vec<f32>>) -> Self::Output {
        self.mat_mult(&rhs)
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
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
