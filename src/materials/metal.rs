use crate::{ray::Ray, Color, Vec3, shapes::HitRecord};

use super::material::Scatter;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz,
        }
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
    ) -> Option<(crate::ray::Ray, Color)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(r.dir()), rec.normal());
        let scattered = Ray::new(
            *rec.p(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if Vec3::dot(scattered.dir(), rec.normal()) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
