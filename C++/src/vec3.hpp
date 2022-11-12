#ifndef VEC3_HPP
#define VEC3_HPP

#include "util.hpp"

#include <iostream>


class Vec3 {
private:
    double e[3];

public:
    Vec3();
    Vec3(double e0, double e1, double e2);

    double x() const;
    double y() const;
    double z() const;

    Vec3 operator-() const;
    Vec3& operator+=(const Vec3 &other);
    Vec3& operator*=(const double &t);
    Vec3& operator/=(const double &t);

    double length() const;
    double length_squared() const;
    bool near_zero() const;

    // Vec3 Utility Functions
    friend inline std::ostream& operator<<(std::ostream &out, const Vec3 &v) {
        return out << v.e[0] << ' ' << v.e[1] << ' ' << v.e[2];
    }

    friend inline Vec3 operator+(const Vec3 &lhs, const Vec3 &rhs) {
        return Vec3(
                lhs.e[0] + rhs.e[0],
                lhs.e[1] + rhs.e[1],
                lhs.e[2] + rhs.e[2]
        );
    }

    friend inline Vec3 operator-(const Vec3 &lhs, const Vec3 &rhs) {
        return Vec3(
                lhs.e[0] - rhs.e[0],
                lhs.e[1] - rhs.e[1],
                lhs.e[2] - rhs.e[2]
        );
    }

    friend inline Vec3 operator*(const Vec3 &lhs, const Vec3 &rhs) {
        return Vec3(
                lhs.e[0] * rhs.e[0],
                lhs.e[1] * rhs.e[1],
                lhs.e[2] * rhs.e[2]
        );
    }

    friend inline Vec3 operator*(const double &lhs, const Vec3 &rhs) {
        return Vec3(
                rhs.e[0] * lhs,
                rhs.e[1] * lhs,
                rhs.e[2] * lhs
        );
    }

    friend inline Vec3 operator*(const Vec3 &lhs, const double &rhs) {
        return Vec3(
                lhs.e[0] * rhs,
                lhs.e[1] * rhs,
                lhs.e[2] * rhs
        );
    }

    friend inline Vec3 operator/(const Vec3 &lhs, const double &rhs) {
        return Vec3(
                lhs.e[0] / rhs,
                lhs.e[1] / rhs,
                lhs.e[2] / rhs
        );
    }

    friend inline double dot(const Vec3 &a, const Vec3 &b) {
        return (a.e[0] * b.e[0]) + (a.e[1] * b.e[1]) + (a.e[2] * b.e[2]);
    }

    friend inline Vec3 cross(const Vec3 &a, const Vec3 &b) {
        return Vec3(
                (a.e[1] * b.e[2]) - (a.e[2] * b.e[1]),
                (a.e[2] * b.e[0]) - (a.e[0] * b.e[2]),
                (a.e[0] * b.e[1]) - (a.e[1] * b.e[0])
        );
    }

    friend inline Vec3 unit_vector(Vec3 v) { return v / v.length(); }

    inline static Vec3 random() {
        return Vec3(random_double(), random_double(), random_double());
    }

    inline static Vec3 random(double min, double max) {
        return Vec3(random_double(min, max), random_double(min, max), random_double(min, max));
    }
};

Vec3 random_in_unit_sphere();
Vec3 random_in_hemisphere(const Vec3 &normal);
Vec3 random_in_unit_disk();
Vec3 random_unit_vector();
Vec3 reflect(const Vec3 &v, const Vec3 &n);
Vec3 refract(const Vec3 &uv, const Vec3 &n, double etai_over_etat);

// Type aliases for Vec3
using Point3 = Vec3;  // 3D point
using Color = Vec3;   // RGB color

#endif //VEC3_HPP