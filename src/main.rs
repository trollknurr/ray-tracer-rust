mod vec3;
mod color;
mod ray;

fn hit_sphere(center: vec3::Point3, radius: f32, ray: &ray::Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = vec3::dot(ray.direction(), ray.direction());
    let b = 2. * vec3::dot(oc, ray.direction());
    let c = vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        return -1.
    } else {
        return  (-b - discriminant.sqrt()) / (2. * a)
    }
}

fn ray_color(r: ray::Ray) -> vec3::Color {
    let t = hit_sphere(vec3::Point3::new(0., 0., -1.), 0.5, &r);
    if t > 0. {
        let n = vec3::unit_vector(r.at(t) - vec3::Vec3::new(0.,0.,-1.));
        return 0.5 * vec3::Color::new(n.x() + 1., n.y() + 1., n.z() + 1.)
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
    
    // Camera
    let viewport_height = 2.0 as f32;
    let viewport_width = (aspect_ratio as f32) * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3::new(0., 0., 0.);
    let horizontal = vec3::Vec3::new(viewport_width, 0., 0.);
    let vertical = vec3::Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - vec3::Vec3::new(0., 0., focal_length);
    
    print!("P3\n{} {}\n255\n", image_width, image_height);
    
    for j in (0..=image_height-1).rev() {
        eprintln!("Scanlines remainging: {}", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1)  as f32;
            let ray = ray::Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(ray);
            color::write_color(pixel_color);
        }
    }
    eprintln!("Done!");
}
