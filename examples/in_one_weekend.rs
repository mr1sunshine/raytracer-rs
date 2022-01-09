use rand::Rng;
use raytracer::{Camera, Color, Material, Point3, Renderer, Scene, Shape, Vec3};
use std::{env, process::exit};

fn random_scene() -> Scene {
    let mut rng = rand::thread_rng();

    let mut world = Scene::default();

    world.add(
        Shape::Sphere {
            center: Point3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
        },
        Material::Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        },
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_vec() * Color::random_vec();
                    Material::Lambertian { albedo }
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Material::Metal { albedo, fuzz }
                } else {
                    Material::Dielectric { ir: 1.5 }
                };

                world.add(
                    Shape::Sphere {
                        center,
                        radius: 0.2,
                    },
                    sphere_material,
                );
            }
        }
    }

    world.add(
        Shape::Sphere {
            center: Point3::new(0.0, 1.0, 0.0),
            radius: 1.0,
        },
        Material::Dielectric { ir: 1.5 },
    );
    world.add(
        Shape::Sphere {
            center: Point3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
        },
        Material::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        },
    );
    world.add(
        Shape::Sphere {
            center: Point3::new(4.0, 1.0, 0.0),
            radius: 1.0,
        },
        Material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    );

    world
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify output file as command line argument");
        exit(-1);
    }

    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: u32 = 10;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    // Camera
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render
    let mut renderer = Renderer::new(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        &args[1],
    )?;
    renderer.render(&camera, &world)?;

    Ok(())
}
