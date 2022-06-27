fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn write_color(pixel_color: crate::vec3::Color, samples_per_pixel: u8) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / (samples_per_pixel as f32);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (256 as f32 * clamp(r, 0.0, 0.999)) as u8;
    let ig = (256 as f32 * clamp(g, 0.0, 0.999)) as u8;
    let ib = (256 as f32 * clamp(b, 0.0, 0.999)) as u8;

    println!("{} {} {}", ir, ig, ib);
}
