#ifndef HITTABLE_LIST_HPP
#define HITTABLE_LIST_HPP

#include "vec3.hpp"
#include "hittable.hpp"

#include <memory>
#include <vector>
#include <optional>

struct HittableList : public Hittable {
    std::vector<std::shared_ptr<Hittable>> objects;

    HittableList() {}
    HittableList(std::shared_ptr<Hittable> obj) { add(obj); }

    void clear() { objects.clear(); }
    void add(std::shared_ptr<Hittable> obj) { objects.push_back(obj); }

    virtual std::optional<HitRecord> hit(const Ray &r, double t_min, double t_max) const override;
};

std::optional<HitRecord> HittableList::hit(const Ray &r, double t_min, double t_max) const {
    std::optional<HitRecord> rec = std::nullopt;
    auto closest_so_far = t_max;

    for (const auto& object : objects) {
        if (std::optional<HitRecord> temp_rec = object->hit(r, t_min, closest_so_far); temp_rec) {
            closest_so_far = temp_rec.value().t;
            rec = temp_rec;
        }
    }

    return rec;
}


#endif //HITTABLE_LIST_HPP
