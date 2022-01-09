use rand::Rng;

use crate::{ray::Ray, Color, Vec3, shapes::HitRecord};

use super::material::Scatter;

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        r: &crate::ray::Ray,
        rec: &HitRecord,
    ) -> Option<(Ray, Color)> {
        let refraction_ratio = if rec.front_face() {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(r.dir());

        let cos_theta = Vec3::dot(&-unit_direction, rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();

        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            Vec3::reflect(&unit_direction, rec.normal())
        } else {
            Vec3::refract(&unit_direction, rec.normal(), refraction_ratio)
        };

        Some((Ray::new(*rec.p(), direction), Color::new(1.0, 1.0, 1.0)))
    }
}
