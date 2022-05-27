use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;
use std::vec::Vec;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for obj in self.objects.iter() {
            match obj.hit(ray, t_min, closest_so_far) {
                Some(record) => {
                    closest_so_far = record.t;
                    rec = Some(record);
                }
                None => (),
            }
        }

        rec
    }
}
