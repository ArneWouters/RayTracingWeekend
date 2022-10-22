use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far: f64 = t_max;

        for object in self.objects.iter() {
            if let Some(h) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = h.t;
                rec = Some(h);
            }
        }

        rec
    }

}
