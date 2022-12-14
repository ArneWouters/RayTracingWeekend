use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::util::*;

#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub use Vec3 as Point3;
pub use Vec3 as Color;

// overload operators
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t
        }

    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Vec3 {
            x: t * self.x,
            y: t * self.y,
            z: t * self.z
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z }
    }
}


// print function
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}


impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn length_squared(self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(a: Vec3, b: Vec3) -> f64 {
        (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere: Vec3 = Vec3::random_in_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;

        (self.x().abs() < EPS) && (self.y().abs() < EPS) && (self.z().abs() < EPS)
    }

    pub fn reflect(a: Vec3, b: Vec3) -> Vec3 {
        a - (2.0 * Vec3::dot(a, b) * b)
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(-uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + (cos_theta * n));
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }
}

pub fn write_color(color: Vec3, samples_per_pixel: u64) {
    println!("{} {} {}",
             (256.0 * (color.x / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64,
             (256.0 * (color.y / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64,
             (256.0 * (color.z / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64
    )
}

