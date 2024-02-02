use rand::{random, thread_rng, Rng};
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

    pub fn r(self) -> f64 {
        self.x
    }
    pub fn g(self) -> f64 {
        self.y
    }
    pub fn b(self) -> f64 {
        self.z
    }

    /// Calculate the dot product of two 3D vectors.
    pub fn dot(&self, other: &Vector3) -> f64 {
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

    /// Generate a random vector in range (0.0, 1.0]
    pub fn random_vector() -> Vector3 {
        Vector3::new(random::<f64>(), random::<f64>(), random::<f64>())
    }

    /// Generate a random vector in range [min, max]
    pub fn random_vector_mm(min: f64, max: f64) -> Vector3 {
        let mut rng = thread_rng();

        Vector3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
    pub fn random_in_unit_disk() -> Vector3 {
        let mut rng = thread_rng();

        loop {
            let rx = rng.gen_range(-1.0..=1.00);
            let ry = rng.gen_range(-1.0..=1.00);
            let r_point = Vector3::new(rx, ry, 0.0);

            if r_point.magnitude() < 1.0 {
                return r_point;
            }
        }
    }

    pub fn random_unit() -> Vector3 {
        Vector3::normalize(&Vector3::random_in_unit_disk())
    }

    pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let on_unit_sphere = Vector3::random_unit();

        return if on_unit_sphere.dot(&normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        };
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

    pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = f64::min(-uv.dot(n), 1.0);
        let r_out_perpendicular = (*uv + (*n * cos_theta)) * etai_over_etat;
        let r_out_parallel = (*n) * -f64::sqrt(f64::abs(1.0 - r_out_perpendicular.magnitude()));

        return r_out_perpendicular + r_out_parallel;
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
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
            x: 0.0 / other,
            y: 0.0 / other,
            z: 0.0 / other,
        }
    }
}

pub type Color = Vector3;
pub type Point3 = Vector3;

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

    // TODO: add tests
}
