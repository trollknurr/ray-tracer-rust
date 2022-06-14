mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use rand::prelude::*;

use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use std::rc::Rc;

fn ray_color(r: &ray::Ray, world: &HittableList) -> vec3::Color {
    match world.hit(r, 0., f32::INFINITY) {
        Some(record) => return 0.5 * (record.normal + vec3::Color::new(1., 1., 1.)),
        None => (),
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    vec3::Color::new(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(vec3::Point3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(
        vec3::Point3::new(0., -100.5, -1.),
        100.,
    )));

    let camera = camera::Camera::new();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..=image_height - 1).rev() {
        eprintln!("Scanlines remainging: {}", j);
        for i in 0..image_width {
            let mut pixel_color = vec3::Color::new(0., 0., 0.);
            for s in 0..samples_per_pixel {
                let ru: f32 = random();
                let rv: f32 = random();
                let u = (i as f32 + ru) / (image_width - 1) as f32;
                let v = (j as f32 + rv) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world);
            }
            color::write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done!");
}
