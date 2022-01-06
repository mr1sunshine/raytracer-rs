use crate::Color;
use crate::utility::clamp;
use std::fs::File;
use std::io::Write;

pub fn write_color(file: &mut File, color: &Color, samples_per_pixel: u32) -> std::io::Result<()> {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    writeln!(file, "{} {} {}", ir, ig, ib)?;
    Ok(())
}
