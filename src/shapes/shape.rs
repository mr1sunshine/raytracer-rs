use crate::{Point3, ray::Ray};

use super::{hittable::{Hittable, HitRecord}, sphere::Sphere};

pub enum Shape {
    Sphere { center: Point3, radius: f64 },
}

impl Hittable for Shape {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Shape::Sphere { center, radius } => {
                let shape = Sphere::new(*center, *radius);
                shape.hit(r, t_min, t_max)
            },
        }
    }
}