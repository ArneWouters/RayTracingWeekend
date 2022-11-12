#ifndef SPHERE_HPP
#define SPHERE_HPP

#include "vec3.hpp"
#include "hittable.hpp"
#include "material.hpp"

#include <cmath>
#include <optional>
#include <memory>

class Sphere : public Hittable {
private:
    Point3 center;
    double radius;
    std::shared_ptr<Material> mat_ptr;

public:
    Sphere() {}
    Sphere(Point3 cen, double r, std::shared_ptr<Material> m) : center(cen), radius(r), mat_ptr(m) {}

    virtual std::optional<HitRecord> hit(const Ray &r, double t_min, double t_max) const override;
};

std::optional<HitRecord> Sphere::hit(const Ray &r, double t_min, double t_max) const {
    Vec3 oc = r.origin() - center;
    auto a = r.direction().length_squared();
    auto half_b = dot(oc, r.direction());
    auto c = oc.length_squared() - (radius * radius);
    auto discriminant = (half_b * half_b) - (a * c);

    if (discriminant < 0.0) return std::nullopt;

    auto sqrtd = sqrt(discriminant);

    // Find the nearest root that lies in the acceptable range.
    auto root = (-half_b - sqrtd) / a;
    if (root < t_min || t_max < root) {
        root = (-half_b + sqrtd) / a;
        if (root < t_min || t_max < root) return std::nullopt;
    }

    Vec3 outward_normal = (r.at(root) - center) / radius;
    HitRecord rec{r.at(root), (r.at(root) - center) / radius, root, false, mat_ptr};
    rec.set_face_normal(r, outward_normal);

    return rec;
}

#endif //SPHERE_HPP
