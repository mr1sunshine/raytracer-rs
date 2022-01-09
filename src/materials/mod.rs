mod dielectric;
mod lambertian;
mod material;
mod metal;

pub type Material = material::Material;
pub use material::Scatter;