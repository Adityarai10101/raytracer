use std::fmt;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};
use crate::rand::Rand;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    vec: [f64; 3],
}

impl Vec3 {
    #[inline]
    pub fn new(one: f64, two: f64, three: f64) -> Self {
        Self {
            vec: [one, two, three],
        }
    }

    pub fn random() -> Self {
        let mut randomizer = Rand::new_with_nanos();
        Self {
            vec: [randomizer.next() as f64, randomizer.next() as f64, randomizer.next() as f64]
        }
    }

    pub fn random_with_randomizer_and_range(randomizer: &mut Rand, range_min: f32, range_max: f32) -> Self {
        Self {
            vec: [
                randomizer.next_with_range(range_min, range_max) as f64, 
                randomizer.next_with_range(range_min, range_max) as f64, 
                randomizer.next_with_range(range_min, range_max) as f64, 
            ]
        }
    }
    
    pub fn random_unit_vector(randomizer: &mut Rand) -> Self {
        loop {
            let rand_vec = Vec3::random_with_randomizer_and_range(randomizer, -1.0, 1.0);
            if (0.00001 < rand_vec.length_squared() && rand_vec.length_squared() <= 1.0) {
                return rand_vec.unit_vector()
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3, randomizer: &mut Rand) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector(randomizer);
        if on_unit_sphere.dot(normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return Vec3::new(0.0, 0.0, 0.0) - on_unit_sphere;
        }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.vec[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.vec[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.vec[2]
    }

    pub fn near_zero(&self) -> bool {
        return (self.x() < 0.000001) && (self.y() < 0.000001) &&  (self.z() < 0.000001);
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.vec[0] * self.vec[0] + self.vec[1] * self.vec[1] + self.vec[2] * self.vec[2]
    }

    #[inline]
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.vec[0] * other.vec[0] + self.vec[1] * other.vec[1] + self.vec[2] * other.vec[2]
    }

    #[inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.vec[1] * other.vec[2] - self.vec[2] * other.vec[1],
            self.vec[2] * other.vec[0] - self.vec[0] * other.vec[2],
            self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0],
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

pub type Point3 = Vec3; // we're basically just making nalgebra atp
pub type Color3 = Vec3; 

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
            self.vec[2] + other.vec[2],
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
            self.vec[2] - other.vec[2],
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
            self.vec[2] * other.vec[2],
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

#[inline]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - 2.0 * dot(v, n) * *n;
}

#[inline]
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min((-*uv).dot(n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * *n;
    r_out_perp + r_out_parallel
}
