mod shape;
mod hittable;
mod sphere;

pub type Shape = shape::Shape;
pub type HitRecord = hittable::HitRecord;
pub use hittable::Hittable;