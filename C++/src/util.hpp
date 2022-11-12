#ifndef UTIL_HPP
#define UTIL_HPP

#include <cmath>
#include <random>

inline double degrees_to_radians(double degrees) { return degrees * M_PI / 180.0; }

inline double random_double(double min = 0.0, double max = 1.0) {
    static std::mt19937 generator;
    std::uniform_real_distribution<double> distribution(min, max);
    return distribution(generator);
}

#endif //UTIL_HPP
