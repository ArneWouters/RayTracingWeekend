use std::io::Write;
use std::rc::Rc;

mod vec;
mod ray;
mod hittable;
mod hittablelist;
mod sphere;
mod camera;
mod material;
mod util;

use vec::{Vec3, Point3, Color, write_color};
use ray::Ray;
use sphere::Sphere;
use hittablelist::{HittableList};
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};
use util::*;

fn ray_color(r: &Ray, world: &HittableList, depth: u64) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let target: Point3 = rec.p + Vec3::random_in_hemisphere(rec.normal);

        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth-1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }

    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);

        ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 50;

    // World
    let mut world: HittableList = HittableList::new();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left.clone());
    let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, mat_left);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.add(Box::new(sphere_ground));
    world.add(Box::new(sphere_center));
    world.add(Box::new(sphere_left));
    world.add(Box::new(sphere_left_inner));
    world.add(Box::new(sphere_right));

    // Camera
    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    const APERTURE: f64 = 2.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        APERTURE,
        dist_to_focus
    );

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        std::io::stderr().flush().expect("Unable to flush stderr");

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f64) + random_double()) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_double()) / ((IMAGE_HEIGHT - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone.");
}

