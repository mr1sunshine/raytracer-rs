use std::{rc::Rc, sync::Arc};

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    Point3, Vec3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let oc = *r.orig() - self.center;
        let a = r.dir().len_squared();
        let half_b = Vec3::dot(&oc, r.dir());
        let c = oc.len_squared() - self.radius.powf(2.0);

        let discriminant = half_b.powf(2.0) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies on the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let normal = (p - self.center) / self.radius;

        Some(HitRecord::new(p, normal, root, r, self.material.clone()))
    }
}
