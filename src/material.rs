use crate::hittable::{HitRecord, Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, Color};

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
