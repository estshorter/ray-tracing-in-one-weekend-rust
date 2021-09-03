// mod vec3;
use crate::vec3::Color;
// use std::io::Write;

// pub fn write_color(
//     out: &mut std::io::BufWriter<std::io::StdoutLock>,
//     mut pixel_color: Color,
//     samples_per_pixel: i32,
// ) {
//     pixel_color /= samples_per_pixel as f64;
//     writeln!(
//         out,
//         "{} {} {}",
//         (256. * pixel_color.x().sqrt().clamp(0., 0.999)) as i32,
//         (256. * pixel_color.y().sqrt().clamp(0., 0.999)) as i32,
//         (256. * pixel_color.z().sqrt().clamp(0., 0.999)) as i32,
//     )
//     .unwrap();
// }

pub fn get_color(pixel_color: Color, samples_per_pixel: i32) -> (i32, i32, i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Replace NaN components with zero. See explanation in Ray Tracing: The Rest of Your Life.
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write the translated [0,255] value of each color component.
    (
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32,
    )
}
