use crate::{hittable::HitRecord, ray::Ray, Color};

pub trait Material: Sync + Send {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
