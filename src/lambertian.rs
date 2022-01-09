use crate::{material::Material, ray::Ray, Color, Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<(crate::ray::Ray, Color)> {
        let mut scatter_direction = *rec.normal() + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = *rec.normal();
        }

        let scattered = Ray::new(*rec.p(), scatter_direction);
        Some((scattered, self.albedo))
    }
}
