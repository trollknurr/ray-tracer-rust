use crate::ray;
use crate::vec3;

pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: vec3::Point3, t: f32) -> Self {
        HitRecord {
            t: t,
            p: p,
            front_face: false,
            normal: Default::default(),
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
