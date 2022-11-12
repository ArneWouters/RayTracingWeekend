#ifndef HITTABLE_HPP
#define HITTABLE_HPP

#include "vec3.hpp"
#include "ray.hpp"

#include <optional>
#include <memory>

class Material;

struct HitRecord {
    Point3 p;
    Vec3 normal;
    double t;
    bool front_face;
    std::shared_ptr<Material> mat_ptr;

    void set_face_normal(const Ray &r, const Vec3 &outward_normal) {
        front_face = dot(r.direction(), outward_normal) < 0.0;
        normal = front_face ? outward_normal : -outward_normal;
    }
};

class Hittable {
public:
    virtual std::optional<HitRecord> hit(const Ray &r, double t_min, double t_max) const = 0;
};

#endif //HITTABLE_HPP
