#ifndef RAY_HPP
#define RAY_HPP

#include "vec3.hpp"

class Ray {
private:
    Point3 orig;
    Vec3 dir;

public:
    Ray() {}
    Ray(const Point3& origin, const Vec3& direction) : orig(origin), dir(direction) {}

    Point3 origin() const  { return orig; }
    Vec3 direction() const { return dir; }

    Point3 at(const double &t) const { return orig + (t * dir); }
};

#endif //RAY_HPP
