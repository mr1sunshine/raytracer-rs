use raytracer::{ray::Ray, write_color, Color, Point3, Vec3};
use std::io::Write;
use std::{env, fs::File, process::exit};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.orig() - center;
    let a = Vec3::dot(&r.dir(), &r.dir());
    let b = 2.0 * Vec3::dot(&oc, &r.dir());
    let c = Vec3::dot(&oc, &oc) - radius.powf(2.0);
    let discriminant = b.powf(2.0) - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    if hit_sphere(sphere_center, 0.5, r) {
        return Color::new(1.0, 0.0,0.0);
    }
    let unit_dir = Vec3::unit_vector(&r.dir());
    let t = 0.5 * (unit_dir.y() + 1.0);

    let start_value = Color::new(1.0, 1.0, 1.0);
    let end_value = Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * start_value + t * end_value
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify output file as command line argument");
        exit(-1);
    }

    let mut file = File::create(&args[1])?;

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render

    // Headers
    writeln!(&mut file, "P3")?;
    writeln!(&mut file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(&mut file, "255")?;

    // Image data
    for j in (0..=(IMAGE_HEIGHT - 1)).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let dir = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray::new(origin, dir);
            let color = ray_color(&r);
            write_color(&mut file, &color)?;
        }
    }

    Ok(())
}
