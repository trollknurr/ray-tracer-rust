use crate::hittable::{HitRecord, Hittable, Material};
use crate::ray::Ray;
use crate::vec3::{dot, Point3};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::new(ray.at(root), root, Rc::clone(&self.material));
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}
