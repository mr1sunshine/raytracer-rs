use std::fs::File;
use std::io::Write;

pub mod ray;
mod vec3;

pub type Vec3 = vec3::Vec3;
pub type Point3 = vec3::Vec3;
pub type Color = vec3::Vec3;

pub fn write_color(file: &mut File, color: &Color) -> std::io::Result<()> {
    let ir = (255.999 * color.x()) as i32;
    let ig = (255.999 * color.y()) as i32;
    let ib = (255.999 * color.z()) as i32;
    writeln!(file, "{} {} {}", ir, ig, ib)?;
    Ok(())
}
