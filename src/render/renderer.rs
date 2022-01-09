use crate::materials::Scatter;
use crate::{hittable::Hittable, ray::Ray, Color, Vec3};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;

use super::camera::Camera;

pub struct Renderer {
    width: i32,
    height: i32,
    samples_per_pixel: u32,
    max_depth: i32,
    output: File,
}

impl Renderer {
    pub fn new(
        width: i32,
        height: i32,
        samples_per_pixel: u32,
        max_depth: i32,
        output_path: &str,
    ) -> std::io::Result<Self> {
        let output = File::create(output_path)?;

        Ok(Self {
            width,
            height,
            samples_per_pixel,
            max_depth,
            output,
        })
    }

    fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) {
            if let Some((scattered, attenuation)) = rec.material().scatter(r, &rec) {
                return attenuation * Self::ray_color(&scattered, world, depth - 1);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let unit_dir = Vec3::unit_vector(r.dir());
        let t = 0.5 * (unit_dir.y() + 1.0);

        let start_value = Color::new(1.0, 1.0, 1.0);
        let end_value = Color::new(0.5, 0.7, 1.0);

        (1.0 - t) * start_value + t * end_value
    }

    fn generate_pixel_color(
        &self,
        column: i32,
        row: i32,
        camera: &Camera,
        world: &dyn Hittable,
    ) -> Color {
        let pixels: Vec<_> = (0..self.samples_per_pixel)
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::thread_rng();

                let u = (column as f64 + rng.gen::<f64>()) / (self.width - 1) as f64;
                let v = (row as f64 + rng.gen::<f64>()) / (self.height - 1) as f64;
                let r = camera.get_ray(u, v);
                Self::ray_color(&r, world, self.max_depth)
            })
            .collect();

        pixels
            .iter()
            .fold(Color::new(0.0, 0.0, 0.0), |sum, &val| sum + val)
    }

    fn generate_pixels(&self, camera: &Camera, world: &dyn Hittable) -> Vec<Vec<Color>> {
        let pb = ProgressBar::new(self.height as u64);
        pb.set_style(ProgressStyle::default_bar().template(
            "{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta_precise})",
        ));
        pb.set_message("Generating image pixels");

        let pixels: Vec<Vec<_>> = (0..self.height)
            .into_par_iter()
            .rev()
            .progress_with(pb)
            .map(|j| {
                (0..self.width)
                    .into_par_iter()
                    .map(|i| self.generate_pixel_color(i, j, camera, world))
                    .collect::<Vec<_>>()
            })
            .collect();

        // pb.finish_with_message("Successfully generated image pixels");

        pixels
    }

    fn write_color(&mut self, color: &Color) -> std::io::Result<()> {
        let mut r = color.x();
        let mut g = color.y();
        let mut b = color.z();

        let scale = 1.0 / (self.samples_per_pixel as f64);
        r = (scale * r).sqrt().clamp(0.0, 0.999);
        g = (scale * g).sqrt().clamp(0.0, 0.999);
        b = (scale * b).sqrt().clamp(0.0, 0.999);

        let ir = (256.0 * r) as i32;
        let ig = (256.0 * g) as i32;
        let ib = (256.0 * b) as i32;

        writeln!(self.output, "{} {} {}", ir, ig, ib)?;
        Ok(())
    }

    fn encode_image(&mut self, pixels: &[Vec<Color>]) -> std::io::Result<()> {
        let pb = ProgressBar::new(self.height as u64);
        pb.set_style(ProgressStyle::default_bar().template(
            "{spinner:.green} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta_precise})",
        ));
        pb.set_message("Encoding image");

        // Image header
        writeln!(&self.output, "P3")?;
        writeln!(&self.output, "{} {}", self.width, self.height)?;
        writeln!(&self.output, "255")?;

        // Image data
        for row in pixels {
            for pixel_color in row {
                self.write_color(pixel_color)?;
            }
            pb.inc(1);
        }

        pb.finish_with_message("Image encoded");
        Ok(())
    }

    pub fn render(&mut self, camera: &Camera, world: &dyn Hittable) -> std::io::Result<()> {
        let now = Instant::now();

        let pixels = self.generate_pixels(camera, world);
        self.encode_image(&pixels)?;

        let elapsed = now.elapsed();
        println!("Rendering took: {:.2?}", elapsed);
        io::stdout().flush().unwrap();

        Ok(())
    }
}
