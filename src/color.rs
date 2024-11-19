use std::io::Write;

use crate::vec3::Vec3;
use crate::util::clamp;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, color: Color, samples_per_pixel: i32) {
    let r = (256.0 * clamp(color.x() * (1.0 / samples_per_pixel as f64), 0.0, 0.999).sqrt()) as i32;
    let g = (256.0 * clamp(color.y() * (1.0 / samples_per_pixel as f64), 0.0, 0.999).sqrt()) as i32;
    let b = (256.0 * clamp(color.z() * (1.0 / samples_per_pixel as f64), 0.0, 0.999).sqrt()) as i32;

    writeln!(out, "{} {} {}", r, g, b).expect("Error when writing color");
}