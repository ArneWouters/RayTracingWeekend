#include "vec3.hpp"
#include "color.hpp"
#include "ray.hpp"
#include "hittable_list.hpp"
#include "sphere.hpp"
#include "camera.hpp"
#include "util.hpp"
#include "material.hpp"

#include <cmath>
#include <limits>
#include <memory>
#include <iostream>


HittableList random_scene() {
    HittableList world;

    auto ground_material = std::make_shared<Lambertian>(Color(0.5, 0.5, 0.5));
    world.add(std::make_shared<Sphere>(Point3(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for (int a = -11; a < 11; a++) {
        for (int b = -11; b < 11; b++) {
            auto choose_mat = random_double();
            Point3 center(a + (0.9 * random_double()), 0.2, b + (0.9 * random_double()));

            if ((center - Point3(4.0, 0.2, 0.0)).length() > 0.9) {
                std::shared_ptr<Material> sphere_material;

                if (choose_mat < 0.8) {
                    // diffuse
                    auto albedo = Color::random() * Color::random();
                    sphere_material = std::make_shared<Lambertian>(albedo);
                    world.add(std::make_shared<Sphere>(center, 0.2, sphere_material));
                } else if (choose_mat < 0.95) {
                    // metal
                    auto albedo = Color::random(0.5, 1.0);
                    auto fuzz = random_double(0.0, 0.5);
                    sphere_material = std::make_shared<Metal>(albedo, fuzz);
                    world.add(std::make_shared<Sphere>(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = std::make_shared<Dielectric>(1.5);
                    world.add(std::make_shared<Sphere>(center, 0.2, sphere_material));
                }
            }
        }
    }

    auto material1 = std::make_shared<Dielectric>(1.5);
    world.add(std::make_shared<Sphere>(Point3(0.0, 1.0, 0.0), 1.0, material1));

    auto material2 = std::make_shared<Lambertian>(Color(0.4, 0.2, 0.1));
    world.add(std::make_shared<Sphere>(Point3(-4.0, 1.0, 0.0), 1.0, material2));

    auto material3 = std::make_shared<Metal>(Color(0.7, 0.6, 0.5), 0.0);
    world.add(std::make_shared<Sphere>(Point3(4.0, 1.0, 0.0), 1.0, material3));

    return world;
}


Color ray_color(const Ray& r, const Hittable &world, int depth) {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if (depth <= 0) return Color(0.0,0.0,0.0);

    if (auto rec = world.hit(r, 0.001, std::numeric_limits<double>::infinity()); rec) {
        if (auto temp = rec.value().mat_ptr->scatter(r, rec.value()); temp) {
            auto [attenuation, scattered] = temp.value();
            return attenuation * ray_color(scattered, world, depth - 1);
        }

        return Color(0.0, 0.0, 0.0);
    }

    Vec3 unit_direction = unit_vector(r.direction());
    auto t = 0.5 * (unit_direction.y() + 1.0);
    return ((1.0 - t) * Color(1.0, 1.0, 1.0)) + (t * Color(0.5, 0.7, 1.0));
}


int main() {
    // Image
    constexpr double ASPECT_RATIO = 3.0 / 2.0;
    constexpr int IMG_WIDTH = 1200;
    constexpr int IMG_HEIGHT = static_cast<int>(IMG_WIDTH / ASPECT_RATIO);
    constexpr int SAMPLES_PER_PIXEL = 500;
    constexpr int MAX_DEPTH = 50;

    // World
    auto world = random_scene();

    // Camera
    Point3 lookfrom(13.0, 2.0, 3.0);
    Point3 lookat(0.0, 0.0, 0.0);
    Vec3 vup(0.0, 1.0, 0.0);
    constexpr double DIST_TO_FOCUS = 10.0;
    constexpr double APERTURE = 0.1;

    Camera cam(lookfrom, lookat, vup, 20, ASPECT_RATIO, APERTURE, DIST_TO_FOCUS);

    // Render
    std::cout << "P3\n" << IMG_WIDTH << ' ' << IMG_HEIGHT << "\n255" << std::endl;

    for (int j = IMG_HEIGHT-1; j >= 0; --j) {
        std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
        for (int i = 0; i < IMG_WIDTH; ++i) {
            Color pixel_color(0.0, 0.0, 0.0);

            for (int s = 0; s < SAMPLES_PER_PIXEL; s++) {
                auto u = (i + random_double()) / (IMG_WIDTH - 1);
                auto v = (j + random_double()) / (IMG_HEIGHT - 1);
                Ray r = cam.get_ray(u, v);
                pixel_color += ray_color(r, world, MAX_DEPTH);
            }

            write_color(std::cout, pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    std::cerr << "\nDone." << std::endl;
    return 0;
}
