mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod util;
mod camera;

use std::io;
use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};
use hittable::{HitRecord, Hittable, HittableList};
use sphere::Sphere;
use util::random_f64;
use camera::Camera;


fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, std::f64::INFINITY, &mut rec) {
        let direction = rec.normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.point, direction), world, depth - 1);
    }

    let unit = r.direction().unit_vector();
    let t = 0.5 * (unit.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)    
}


fn main() {
    const IMAGE_WIDTH: i32 = 1280;
    const IMAGE_HEIGHT: i32 = 720;
    const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 10;

    // Camera

    let camera = Camera::new();
    
    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    

    // PPM Print

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines in progress: {} / {} ({:.2} %)", j + 1, IMAGE_HEIGHT, (j+1) as f64 / IMAGE_HEIGHT as f64 * 100.0);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = ((IMAGE_HEIGHT - j) as f64 + random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprint!("\nDone. \n");
}