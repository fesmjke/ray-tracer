use crate::float_eq::{ApproxEq, EPSILON};
use crate::matrices::Matrix4;
use crate::point::Point;
use crate::transformations::Transformable;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Self { x, y, z }
    }

    /// Calculate the dot product of two 3D vectors.
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // TODO: move to somewhere else, temporary solution
    pub fn dot_point(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate the cross production of two 3D vectors.
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Calculate the magnitude (length) of a vector.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Normalize a vector to have a magnitude of 1.
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    /// Checks vector is close to zero
    pub fn near_zero(&self) -> bool {
        let almost_zero = 1e-8;

        return self.x.abs() < almost_zero
            && self.y.abs() < almost_zero
            && self.z.abs() < almost_zero;
    }

    /// Reflects a 3D vector off a surface defined by a normal vector.
    pub fn reflect(&self, normal: &Vector3) -> Vector3 {
        *self - *normal * 2.0 * self.dot(normal)
    }

    // TODO: rewrite / change uv: to self?
    pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = f64::min(-uv.dot(n), 1.0);
        let r_out_perpendicular = (*uv + (*n * cos_theta)) * etai_over_etat;
        let r_out_parallel = (*n) * -f64::sqrt(f64::abs(1.0 - r_out_perpendicular.magnitude()));

        return r_out_perpendicular + r_out_parallel;
    }
}

impl Transformable for Vector3 {
    fn transform(self, transformation: &Matrix4) -> Self {
        transformation.clone() * self
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(&other)
    }
}

impl ApproxEq for Vector3 {
    fn approx_eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn vector_creation() {
        let vector_default = Vector3::default();
        let vector_new = Vector3::new(0.0, 0.0, 0.0);

        assert_eq!(vector_new, vector_default);
        assert_eq!(vector_new.z, vector_default.z);
    }

    #[test]
    fn vector_addition() {
        let vector_a = Vector3::new(0.0, 1.0, 2.0);
        let vector_b = Vector3::new(0.1, 1.1, 2.2);

        let vector_addition = vector_a + vector_b;
        assert_eq!(vector_addition, Vector3::new(0.1, 2.1, 4.2));

        let vector_addition = vector_b + vector_a;
        assert_eq!(vector_addition, Vector3::new(0.1, 2.1, 4.2));

        let vector_addition = vector_a + vector_a;
        assert_eq!(vector_addition, Vector3::new(0.0, 2.0, 4.0));
    }

    #[test]
    fn vector_subtraction() {
        let vector_a = Vector3::new(-2.5, 1.3, 0.0);
        let vector_b = Vector3::new(0.4, 1.3, 2.2);

        let expected_vector = Vector3::new(-2.9, 0.0, -2.2);
        let vector_subtraction = vector_a - vector_b;
        assert_eq!(expected_vector, vector_subtraction);

        let vector_subtraction = vector_b - vector_a;
        let expected_vector = Vector3::new(2.9, 0.0, 2.2);
        assert_eq!(expected_vector, vector_subtraction);

        let vector_subtraction = vector_a - vector_a;
        let expected_vector = Vector3::new(0.0, 0.0, 0.0);
        assert_eq!(expected_vector, vector_subtraction);
    }

    #[test]
    fn vector_multiplication() {
        let vector_a = Vector3::new(1.0, 2.0, 0.0);
        let vector_b = Vector3::new(1.5, 4.0, 5.0);

        let expected_vector = Vector3::new(1.5, 8.0, 0.0);
        let vector_multiplication = vector_a * vector_b;
        assert_eq!(expected_vector, vector_multiplication);

        let expected_vector = Vector3::new(1.5, 8.0, 0.0);
        let vector_multiplication = vector_b * vector_a;
        assert_eq!(expected_vector, vector_multiplication);

        let expected_vector = Vector3::new(1.0, 4.0, 0.0);
        let vector_multiplication = vector_a * vector_a;
        assert_eq!(expected_vector, vector_multiplication);
    }

    #[test]
    fn vector_multiplication_scalar() {
        let vector_a = Vector3::new(1.0, 2.0, 3.0);

        let expected_vector = Vector3::new(2.0, 4.0, 6.0);
        let vector_multiplication = vector_a * 2.0;
        assert_eq!(expected_vector, vector_multiplication);
    }

    #[test]
    fn vector_division_scalar() {
        let vector_a = Vector3::new(2.0, 2.0, 4.0);

        let expected_vector = Vector3::new(1.0, 1.0, 2.0);
        let vector_division = vector_a / 2.0;
        assert_eq!(expected_vector, vector_division);
    }

    #[test]
    fn vector_magnitude() {
        let vector_a = Vector3::new(2.0, 2.0, 4.0);

        let expected_magnitude = 24.0_f64.sqrt();
        let vector_magnitude = vector_a.magnitude();
        assert_eq!(expected_magnitude, vector_magnitude);
    }

    #[test]
    fn normalize_vector() {
        let vector_a = Vector3::new(4.0, 0.0, 0.0);

        let expected_unit_vector = Vector3::new(1.0, 0.0, 0.0);
        let unit_vector = vector_a.normalize();

        assert_eq!(expected_unit_vector, unit_vector);
    }

    #[test]
    fn vector_negative() {
        let vector_a = Vector3::new(2.0, 2.0, 4.0);

        let expected_vector = Vector3::new(-2.0, -2.0, -4.0);

        assert_eq!(expected_vector, -vector_a);
    }

    #[test]
    fn vector_cross_production() {
        let vector_a = Vector3::new(1.0, 3.0, 4.0);
        let vector_b = Vector3::new(2.0, 7.0, -5.0);

        let expected_vector = Vector3::new(-43.0, 13.0, 1.0);
        let cross_vector_production = vector_a.cross(&vector_b);

        assert_eq!(expected_vector, cross_vector_production);
    }

    #[test]
    fn vector_dot_production() {
        let vector_a = Vector3::new(2.0, 4.0, 6.0);
        let vector_b = Vector3::new(1.0, 3.0, 5.0);

        let expected_dot = 44.0;
        let dot_vector_production = vector_a.dot(&vector_b);

        assert_eq!(expected_dot, dot_vector_production);
    }

    #[test]
    fn normalize_vector_approx() {
        let vector_a = Vector3::new(1.0, 2.0, 3.0);

        let expected_unit_vector = Vector3::new(0.2672612, 0.5345224, 0.8017837);
        let unit_vector = vector_a.normalize();

        assert_eq!(expected_unit_vector, unit_vector);
    }
}
