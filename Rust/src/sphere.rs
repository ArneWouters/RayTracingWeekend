use crate::vec::{Vec3, Point3};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(cen: Point3, r:f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            p: p,
            normal: (p - self.center) / self.radius,
            t: root,
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }

}
