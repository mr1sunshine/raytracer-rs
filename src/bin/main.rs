use rand::Rng;
use raytracer::{
    camera::Camera,
    color::write_color,
    hittable::Hittable,
    hittable_list::{self, HittableList},
    material::Material,
    ray::Ray,
    sphere::Sphere,
    Color, Dielectric, Lambertian, Metal, Point3, Vec3,
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

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::default();

    let ground_material = Rc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let range = 0.0..0.1;
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(range.clone());
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(range.clone()),
                0.2,
                b as f64 + 0.9 * rng.gen_range(range.clone()),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_vec() * Color::random_vec();
                    Rc::new(Lambertian::new(&albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Rc::new(Metal::new(&albedo, fuzz))
                } else {
                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Rc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
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
