use crate::ray;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Color;
use std::rc::Rc;

pub trait Material {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> ScatterResult;
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
    pub is_scatter: bool,
}

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: vec3::Point3, t: f32, material: Rc<dyn Material>) -> Self {
        HitRecord {
            t: t,
            p: p,
            front_face: false,
            normal: Default::default(),
            material: material,
        }
    }
    pub fn set_face_normal(&mut self, ray: &ray::Ray, outward_normal: &vec3::Vec3) {
        self.front_face = vec3::dot(&ray.direction(), outward_normal) < 0.;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
