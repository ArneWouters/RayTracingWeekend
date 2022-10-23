use crate::vec::{Color, Vec3};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::util::*;

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}



pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}



pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        if f < 1.0 {
            Metal {
                albedo: a,
                fuzz: f
            }
        } else {
            Metal {
                albedo: a,
                fuzz: 1.0
            }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(r.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + (self.fuzz * Vec3::random_in_unit_sphere()));

        if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}



pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric { ir: index_of_refraction }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;

        r0 + ((1.0 - r0) * (1.0 - cosine).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_rate = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r.direction().unit_vector();
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();
        let cannot_refract = refraction_rate * sin_theta > 1.0;
        let will_reflect = Self::reflectance(cos_theta, refraction_rate) > random_double();

        let direction = if cannot_refract || will_reflect {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_rate)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.p, direction)))
    }
}

