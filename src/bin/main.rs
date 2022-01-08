use indicatif::{ProgressBar, ProgressStyle};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use rayon::prelude::*;
use raytracer::{
    camera::Camera, color::write_color, hittable::Hittable, hittable_list::HittableList,
    material::Material, ray::Ray, sphere::Sphere, Color, Dielectric, Lambertian, Metal, Point3,
    Vec3,
};
use std::{env, fs::File, io::Write, process::exit, sync::Arc};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material().scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
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

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let between = Uniform::from(0.0..0.1);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = between.sample(&mut rng);
            let center = Point3::new(
                a as f64 + 0.9 * between.sample(&mut rng),
                0.2,
                b as f64 + 0.9 * between.sample(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_vec() * Color::random_vec();
                    Arc::new(Lambertian::new(&albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(&albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}

fn generate_pixel_color(
    samples_per_pixel: usize,
    column: i32,
    row: i32,
    image_width: i32,
    image_height: i32,
    camera: &Camera,
    world: &dyn Hittable,
    max_depth: i32,
) -> Color {
    let pixels: Vec<_> = (0..samples_per_pixel)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();

            let u = (column as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
            let v = (row as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
            let r = camera.get_ray(u, v);
            ray_color(&r, world, max_depth)
        })
        .collect();

    pixels
        .iter()
        .fold(Color::new(0.0, 0.0, 0.0), |sum, &val| sum + val)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify output file as command line argument");
        exit(-1);
    }

    let mut file = File::create(&args[1])?;

    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    // Camera
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    // Headers
    writeln!(&mut file, "P3")?;
    writeln!(&mut file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(&mut file, "255")?;

    let pb = ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta_precise})"));
        // .progress_chars("#>-"));

    // Image data
    let pixels: Vec<Vec<_>> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .map(|j| {
            let row: Vec<_> = (0..IMAGE_WIDTH)
                .into_par_iter()
                .map(|i| {
                    pb.inc(1);
                    generate_pixel_color(
                        SAMPLES_PER_PIXEL as usize,
                        i,
                        j,
                        IMAGE_WIDTH,
                        IMAGE_HEIGHT,
                        &cam,
                        &world,
                        MAX_DEPTH,
                    )
                })
                .collect();

            row
        })
        .collect();

    for row in &pixels {
        for pixel_color in row {
            write_color(&mut file, pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }

    Ok(())
}
