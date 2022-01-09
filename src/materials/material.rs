use crate::{ray::Ray, shapes::HitRecord, Color};

use super::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

pub trait Scatter: Sync + Send {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone, Copy)]
pub enum Material {
    Dielectric { ir: f64 },
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Scatter for Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Dielectric { ir } => {
                let material = Dielectric::new(*ir);
                material.scatter(r, rec)
            }
            Material::Lambertian { albedo } => {
                let material = Lambertian::new(albedo);
                material.scatter(r, rec)
            }
            Material::Metal { albedo, fuzz } => {
                let material = Metal::new(albedo, *fuzz);
                material.scatter(r, rec)
            }
        }
    }
}
