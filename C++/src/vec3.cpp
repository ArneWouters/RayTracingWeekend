#include "vec3.hpp"

#include <cmath>


Vec3::Vec3() : e{0.0, 0.0, 0.0} {}

Vec3::Vec3(double e0, double e1, double e2) : e{e0, e1, e2} {}

double Vec3::x() const { return e[0]; }

double Vec3::y() const { return e[1]; }

double Vec3::z() const { return e[2]; }

Vec3 Vec3::operator-() const { return Vec3(-e[0], -e[1], -e[2]); }

Vec3& Vec3::operator+=(const Vec3 &other) {
    e[0] += other.e[0];
    e[1] += other.e[1];
    e[2] += other.e[2];
    return *this;
}

Vec3& Vec3::operator*=(const double &t) {
    e[0] *= t;
    e[1] *= t;
    e[2] *= t;
    return *this;
}

Vec3& Vec3::operator/=(const double &t) {
    e[0] /= t;
    e[1] /= t;
    e[2] /= t;
    return *this;
}

double Vec3::length() const { return sqrt(length_squared()); }

double Vec3::length_squared() const {
    return (e[0] * e[0]) + (e[1] * e[1]) + (e[2] * e[2]);
}

bool Vec3::near_zero() const {
    // Return true if the vector is close to zero in all dimensions.
    constexpr double s = 1e-8;
    return (fabs(e[0]) < s) && (fabs(e[1]) < s) && (fabs(e[2]) < s);
}

Vec3 random_in_unit_sphere() {
    while (true) {
        auto p = Vec3::random(-1.0, 1.0);
        if (p.length_squared() >= 1.0) continue;
        return p;
    }
}

Vec3 random_in_hemisphere(const Vec3& normal) {
    Vec3 in_unit_sphere = random_in_unit_sphere();
    if (dot(in_unit_sphere, normal) > 0.0) // In the same hemisphere as the normal
        return in_unit_sphere;
    else
        return -in_unit_sphere;
}

Vec3 random_in_unit_disk() {
    while (true) {
        auto p = Vec3(random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0);
        if (p.length_squared() >= 1) continue;
        return p;
    }
}

Vec3 random_unit_vector() { return unit_vector(random_in_unit_sphere()); }

Vec3 reflect(const Vec3 &v, const Vec3 &n) { return v - (2.0 * dot(v,n) * n); }

Vec3 refract(const Vec3 &uv, const Vec3 &n, double etai_over_etat) {
    auto cos_theta = fmin(dot(-uv, n), 1.0);
    Vec3 r_out_perp =  etai_over_etat * (uv + cos_theta*n);
    Vec3 r_out_parallel = -sqrt(fabs(1.0 - r_out_perp.length_squared())) * n;
    return r_out_perp + r_out_parallel;
}

