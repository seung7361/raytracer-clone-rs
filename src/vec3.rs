use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::util::{random_f64, random_f64_range};

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random() -> Vec3 {
        Vec3::new(
            random_f64(),
            random_f64(),
            random_f64(),
        )
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_f64_range(min, max),
            random_f64_range(min, max),
            random_f64_range(min, max),
        )
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.x() + self.y * rhs.y() + self.z * rhs.z()
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z() - self.z * rhs.y(),
            self.z * rhs.x() - self.x * rhs.z(),
            self.x * rhs.y() - self.y * rhs.x()
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Point: {} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_vec3_length() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length(), 3.0);
    }

    #[test]
    fn test_vec3_length_squared() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length_squared(), 9.0);
    }

    #[test]
    fn test_vec3_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, -5.0, 6.0);
        assert_eq!(v1.dot(v2), 12.0);
    }

    #[test]
    fn test_vec3_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let cross = v1.cross(v2);
        assert_eq!(cross.x(), -3.0);
        assert_eq!(cross.y(), 6.0);
        assert_eq!(cross.z(), -3.0);
    }

    #[test]
    fn test_vec3_unit_vector() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let unit = v.unit_vector();
        assert_eq!(unit.x(), 1.0 / 3.0);
        assert_eq!(unit.y(), 2.0 / 3.0);
        assert_eq!(unit.z(), 2.0 / 3.0);
    }

    #[test]
    fn test_vec3_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let sum = v1 + v2;
        assert_eq!(sum.x(), 5.0);
        assert_eq!(sum.y(), 7.0);
        assert_eq!(sum.z(), 9.0);
    }

    #[test]
    fn test_vec3_sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let diff = v1 - v2;
        assert_eq!(diff.x(), -3.0);
        assert_eq!(diff.y(), -3.0);
        assert_eq!(diff.z(), -3.0);
    }

    #[test]
    fn test_vec3_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let product = v * 2.0;
        assert_eq!(product.x(), 2.0);
        assert_eq!(product.y(), 4.0);
        assert_eq!(product.z(), 6.0);
    }

    #[test]
    fn test_vec3_div() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let quotient = v / 2.0;
        assert_eq!(quotient.x(), 1.0);
        assert_eq!(quotient.y(), 2.0);
        assert_eq!(quotient.z(), 3.0);
    }

    #[test]
    fn test_vec3_neg() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let neg = -v;
        assert_eq!(neg.x(), -1.0);
        assert_eq!(neg.y(), 2.0);
        assert_eq!(neg.z(), -3.0);
    }
}
