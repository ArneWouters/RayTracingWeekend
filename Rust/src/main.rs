use std::io::Write;
use rand::Rng;

mod vec;
mod ray;
mod hittable;
mod hittablelist;
mod sphere;
mod camera;

use vec::{Vec3, Point3, Color, write_color};
use ray::Ray;
use sphere::Sphere;
use hittablelist::{HittableList};
use camera::Camera;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    let mut rng = rand::thread_rng();

    // World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        std::io::stderr().flush().expect("Unable to flush stderr");

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f64) + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}

