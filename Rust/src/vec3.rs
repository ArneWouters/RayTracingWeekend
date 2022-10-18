use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


// overload operators
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
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

    fn div(self, other: f64) -> Self {
        return (1.0 / other) * self;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, n: f64) {
        self.x /= n;
        self.y /= n;
        self.z /= n;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 { x: self * v.x, y: self * v.y, z: self * v.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, n: f64) -> Self {
        Vec3 { x: n * self.x, y: n * self.y, z: n * self.z }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, n: f64) {
        self.x *= n;
        self.y *= n;
        self.z *= n;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
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
        Vec3 { x: x, y: y, z: z }
    }

    pub fn length(&self) -> f64 {
        return (self.x * self.x) + (self.y + self.y) + (self.z * self.z);
    }

    pub fn length_squared(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
        return (a.x * b.x) + (a.y * b.y) + (a.z * b.z);
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        return Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}

pub fn write_color(color: Vec3) {
    println!("{} {} {}",
             (255.999 * color.x) as i64,
             (255.999 * color.y) as i64,
             (255.999 * color.z) as i64
    )
}

