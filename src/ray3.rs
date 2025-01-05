use crate::vec3::{Vec3, Point3};

pub struct Ray3 {
    origin: Point3,
    direction: Vec3
}

impl Ray3 {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.unit_vector()
        }
    }

    pub fn empty_new() -> Self {
        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Point3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
    
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
