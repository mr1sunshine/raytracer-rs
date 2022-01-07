use crate::{hittable::HitRecord, Color, ray::Ray};

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
