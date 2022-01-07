use std::ptr::NonNull;

use crate::{Color, material::Material, Vec3, ray::Ray};

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: &Color) -> Self { Self { albedo: *albedo } }
}

impl Material for Metal {
    fn scatter(&self, r: &crate::ray::Ray, rec: &crate::hittable::HitRecord) -> Option<(crate::ray::Ray, Color)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r.dir()), &rec.normal());
        let scattered = Ray::new(rec.p(), reflected);
        if Vec3::dot(&scattered.dir(), &rec.normal()) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
