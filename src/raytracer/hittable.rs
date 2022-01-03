use crate::{ray::Ray, Point3, Vec3};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, outward_normal: Vec3, t: f64, r: &Ray) -> Self {
        let front_face = Vec3::dot(&r.dir(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }

    /// Get the hit record's p.
    pub fn p(&self) -> Vec3 {
        self.p
    }

    /// Get the hit record's normal.
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// Get the hit record's t.
    pub fn t(&self) -> f64 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
