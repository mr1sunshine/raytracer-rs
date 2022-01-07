use rand::Rng;
use raytracer::{
    camera::Camera, color::write_color, hittable::Hittable, hittable_list::HittableList, ray::Ray,
    sphere::Sphere, Color, Point3, Vec3, Lambertian, Metal, Dielectric,
};
use std::{env, fs::File, io::Write, process::exit, rc::Rc};

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

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

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
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Dielectric::new(1.5));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Camera
    let cam = Camera::new();

    // Render

    // Headers
    writeln!(&mut file, "P3")?;
    writeln!(&mut file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(&mut file, "255")?;

    // Image data
    for j in (0..=(IMAGE_HEIGHT - 1)).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(&mut file, &pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }

    Ok(())
}
