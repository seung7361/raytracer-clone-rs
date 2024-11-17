use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, color: Color) {
    let r = (255.999 * color.x()) as i32;
    let g = (255.999 * color.y()) as i32;
    let b = (255.999 * color.z()) as i32;

    writeln!(out, "{} {} {}", r, g, b).expect("Error when writing color");
}