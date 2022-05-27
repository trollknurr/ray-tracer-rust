pub fn write_color(pixel_color: crate::vec3::Color) {
    let ir = (255.999 * pixel_color.x()) as u8;
    let ig = (255.999 * pixel_color.y()) as u8;
    let ib = (255.999 * pixel_color.z()) as u8;

    println!("{} {} {}", ir, ig, ib);
}
