use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;

        let a = r.direction().dot(r.direction());
        let b = 2.0 * oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let disc: f64 = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return false;
        }

        let mut root = (-b - disc.sqrt()) / (2.0 * a);
        if root <= t_min || root >= t_max {
            root = (-b + disc.sqrt()) / (2.0 * a);
        }

        if root <= t_min || root >= t_max {
            return false;
        }

        rec.t = root;
        rec.point = r.at(rec.t);
        rec.normal = (rec.point - self.center).unit_vector();

        true
    }
}