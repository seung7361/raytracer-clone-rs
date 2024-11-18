mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;

use std::io;
use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

fn sphere_collision(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;

    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;

    let disc: f64 = b * b - 4.0 * a * c;
    if disc < 0.0 {
        -1.0
    } else {
        (-b - disc.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = sphere_collision(Point3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        // collided?
        let normal = (r.at(t) - Point3::new(0.0, 0.0, -1.0)).unit_vector(); // normal vector from the circle center
        
        return 0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    } else {
        // did not collide?
        let unit = r.direction().unit_vector();
        let t = 0.5 * (unit.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    const IMAGE_WIDTH: i32 = 1280;
    const IMAGE_HEIGHT: i32 = 720;
    const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    

    // Camera (for testing)
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines in progress: {} / {} ", j + 1, IMAGE_HEIGHT);

        for i in 0..IMAGE_WIDTH { 
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = (IMAGE_HEIGHT - j) as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let color = ray_color(&r);
            color::write_color(&mut io::stdout(), color);
        }
    }

    eprint!("\nDone. \n");
}