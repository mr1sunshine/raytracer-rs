use std::f64::consts::PI;

use crate::{ray::Ray, Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = Self::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        let origin = look_from;

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let direction =
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin;
        Ray::new(self.origin, direction)
    }

    fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }
}
