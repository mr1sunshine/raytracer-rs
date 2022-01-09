use crate::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    /// Get the ray's orig.
    pub fn orig(&self) -> &Point3 {
        &self.orig
    }

    /// Get the ray's dir.
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
}
