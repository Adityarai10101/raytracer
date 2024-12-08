use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};
use std::fmt;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    vec: [f64; 3]
}

impl Vec3 {
    #[inline]
    pub fn new(one: f64, two: f64, three: f64) -> Self {
        Self { vec: [one, two, three] }
    }

    #[inline]
    pub fn x(&self) -> f64 { self.vec[0] }

    #[inline]
    pub fn y(&self) -> f64 { self.vec[1] }

    #[inline]
    pub fn z(&self) -> f64 { self.vec[2] }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.vec[0] * self.vec[0] +
        self.vec[1] * self.vec[1] +
        self.vec[2] * self.vec[2]
    }

    #[inline]
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.vec[0] * other.vec[0] +
        self.vec[1] * other.vec[1] +
        self.vec[2] * other.vec[2]
    }

    [inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.vec[1] * other.vec[2] - self.vec[2] * other.vec[1],
            self.vec[2] * other.vec[0] - self.vec[0] * other.vec[2],
            self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0]
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

pub type Point3 = Vec3; // we're basically just making nalgebra atp


impl Index<usize> for Vec3 {
    type Output = f64;
    
    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.vec[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.vec[idx]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn neg(self) -> Self::Output {
        Vec3::new(-self.vec[0], -self.vec[1], -self.vec[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn add(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.vec[0] + other.vec[0],
            self.vec[1] + other.vec[1],
            self.vec[2] + other.vec[2]
        )
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Vec3) {
        self.vec[0] += other.vec[0];
        self.vec[1] += other.vec[1];
        self.vec[2] += other.vec[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.vec[0] - other.vec[0],
            self.vec[1] - other.vec[1],
            self.vec[2] - other.vec[2]
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.vec[0] * other.vec[0],
            self.vec[1] * other.vec[1],
            self.vec[2] * other.vec[2]
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, t: f64) -> Self::Output {
        Vec3::new(self.vec[0] * t, self.vec[1] * t, self.vec[2] * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    
    #[inline]
    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    
    #[inline]
    fn div(self, t: f64) -> Self::Output {
        let k = 1.0 / t;
        Vec3::new(self.vec[0] * k, self.vec[1] * k, self.vec[2] * k)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.vec[0], self.vec[1], self.vec[2])
    }
}

#[inline]
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.dot(v)
}

#[inline]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    u.cross(v)
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    v.unit_vector()
}
