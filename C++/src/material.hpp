#ifndef MATERIAL_HPP
#define MATERIAL_HPP

#include "vec3.hpp"
#include "ray.hpp"
#include "util.hpp"

#include <optional>

struct HitRecord;

class Material {
public:
    virtual std::optional<std::pair<Color, Ray>> scatter(const Ray &r, const HitRecord &rec) const = 0;
};

class Lambertian : public Material {
public:
    Color albedo;

    Lambertian(const Color& a) : albedo(a) {}

    virtual std::optional<std::pair<Color, Ray>> scatter(const Ray &r_in, const HitRecord &rec) const override {
        auto scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if (scatter_direction.near_zero()) scatter_direction = rec.normal;

        return {{albedo, Ray(rec.p, scatter_direction)}};
    }
};

class Metal : public Material {
public:
    Color albedo;
    double fuzz;

    Metal(const Color& a, double f) : albedo(a), fuzz(f < 1.0 ? f : 1.0) {}

    virtual std::optional<std::pair<Color, Ray>> scatter(const Ray &r_in, const HitRecord &rec) const override {
        Vec3 reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        auto scattered = Ray(rec.p, reflected + (fuzz * random_in_unit_sphere()));
        if (dot(scattered.direction(), rec.normal) > 0.0) return {{albedo, scattered}};
        else return std::nullopt;
    }
};

class Dielectric : public Material {
public:
    double ir;   // Index of Refraction

    Dielectric(double index_of_refraction) : ir(index_of_refraction) {}

    virtual std::optional<std::pair<Color, Ray>> scatter(const Ray &r_in, const HitRecord &rec) const override {
        double refraction_ratio = rec.front_face ? (1.0 / ir) : ir;
        Vec3 unit_direction = unit_vector(r_in.direction());
        double cos_theta = fmin(dot(-unit_direction, rec.normal), 1.0);
        double sin_theta = sqrt(1.0 - (cos_theta * cos_theta));

        bool cannot_refract = refraction_ratio * sin_theta > 1.0;
        Vec3 direction;

        if (cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double())
            direction = reflect(unit_direction, rec.normal);
        else
            direction = refract(unit_direction, rec.normal, refraction_ratio);

        return {{Color(1.0, 1.0, 1.0), Ray(rec.p, direction)}};
    }

private:
    static double reflectance(double cosine, double ref_idx) {
        // Use Schlick's approximation for reflectance.
        auto r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + ((1.0 - r0) * pow((1.0 - cosine),5.0));
    }
};

#endif //MATERIAL_HPP
