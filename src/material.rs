use std::cmp;

use crate::hittable::{HitRecord, Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::{dot, random_unit_vector, reflect, unit_vector, Color, Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> ScatterResult {
        let mut scatter_direction = record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        ScatterResult {
            scattered: Ray::new(record.p, scatter_direction),
            attenuation: self.albedo,
            is_scatter: true,
        }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let t = if fuzz > 1. { 1. } else { fuzz };
        Self { albedo, fuzz: t }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> ScatterResult {
        let reflected = reflect(&unit_vector(r_in.direction()), &record.normal);
        let scattered = Ray::new(record.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let is_scatter = dot(&scattered.direction(), &record.normal) > 0.;
        ScatterResult {
            attenuation: self.albedo,
            scattered: scattered,
            is_scatter: is_scatter,
        }
    }
}
