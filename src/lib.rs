pub mod hittable;
pub mod hittable_list;
mod ray;
pub mod sphere;

mod vec3;
pub type Vec3 = vec3::Vec3;
pub type Point3 = vec3::Vec3;
pub type Color = vec3::Vec3;

mod render;
pub type Renderer = render::renderer::Renderer;
pub type Camera = render::camera::Camera;

mod materials;
pub type Material = materials::Material;