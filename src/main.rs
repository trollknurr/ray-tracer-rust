mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

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

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(vec3::Point3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(
        vec3::Point3::new(0., -100.5, -1.),
        100.,
    )));

    // Camera
    let viewport_height = 2.0 as f32;
    let viewport_width = (aspect_ratio as f32) * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::new(0., 0., 0.);
    let horizontal = vec3::Vec3::new(viewport_width, 0., 0.);
    let vertical = vec3::Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - vec3::Vec3::new(0., 0., focal_length);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..=image_height - 1).rev() {
        eprintln!("Scanlines remainging: {}", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;
            let ray = ray::Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(&ray, &world);
            color::write_color(pixel_color);
        }
    }
    eprintln!("Done!");
}
