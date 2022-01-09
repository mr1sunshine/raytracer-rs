pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;

mod vec3;
pub type Vec3 = vec3::Vec3;
pub type Point3 = vec3::Vec3;
pub type Color = vec3::Vec3;

pub mod material;
mod lambertian;
pub type Lambertian = lambertian::Lambertian;
mod metal;
pub type Metal = metal::Metal;
mod dielectric;
pub type Dielectric = dielectric::Dielectric;

mod render;
pub type Renderer = render::renderer::Renderer;
pub type Camera = render::camera::Camera;