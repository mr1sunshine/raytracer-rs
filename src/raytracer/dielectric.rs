use crate::{material::Material, ray::Ray, Color, Vec3};

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<(Ray, Color)> {
        let refraction_ratio = if rec.front_face() {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(r.dir());
        let refracted = Vec3::refract(&unit_direction, rec.normal(), refraction_ratio);
        Some((Ray::new(*rec.p(), refracted), Color::new(1.0, 1.0, 1.0)))
    }
}
