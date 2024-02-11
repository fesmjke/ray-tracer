use crate::float_eq::{ApproxEq, EPSILON};
use crate::vector::Vector3;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Self { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn zero() -> Self {
        Point::default()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(&other)
    }
}

impl ApproxEq for Point {
    fn approx_eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self, other: Vector3) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Point> for Vector3 {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, other: Vector3) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn point_creation() {
        let point_default = Point::default();
        let point_new = Point::new(0.0, 0.0, 0.0);

        assert_eq!(point_new, point_default);
        assert_eq!(point_new.z, point_default.z);
    }

    #[test]
    fn point_addition() {
        let point_a = Point::new(0.0, 1.0, 2.0);
        let point_b = Point::new(0.1, 1.1, 2.2);

        let point_addition = point_a + point_b;
        assert_eq!(point_addition, Point::new(0.1, 2.1, 4.2));

        let point_addition = point_b + point_a;
        assert_eq!(point_addition, Point::new(0.1, 2.1, 4.2));

        let point_addition = point_a + point_a;
        assert_eq!(point_addition, Point::new(0.0, 2.0, 4.0));
    }

    #[test]
    fn point_subtraction() {
        let point_a = Point::new(-2.5, 1.3, 0.0);
        let point_b = Point::new(0.4, 1.3, 2.2);

        let expected_point = Point::new(-2.9, 0.0, -2.2);
        let point_subtraction = point_a - point_b;
        assert_eq!(expected_point, point_subtraction);

        let point_subtraction = point_b - point_a;
        let expected_point = Point::new(2.9, 0.0, 2.2);
        assert_eq!(expected_point, point_subtraction);

        let point_subtraction = point_a - point_a;
        let expected_point = Point::new(0.0, 0.0, 0.0);
        assert_eq!(expected_point, point_subtraction);
    }

    #[test]
    fn point_multiplication() {
        let point_a = Point::new(1.0, 2.0, 0.0);
        let point_b = Point::new(1.5, 4.0, 5.0);

        let expected_point = Point::new(1.5, 8.0, 0.0);
        let point_multiplication = point_a * point_b;
        assert_eq!(expected_point, point_multiplication);

        let expected_point = Point::new(1.5, 8.0, 0.0);
        let point_multiplication = point_b * point_a;
        assert_eq!(expected_point, point_multiplication);

        let expected_point = Point::new(1.0, 4.0, 0.0);
        let point_multiplication = point_a * point_a;
        assert_eq!(expected_point, point_multiplication);
    }

    #[test]
    fn point_multiplication_scalar() {
        let point_a = Point::new(1.0, 2.0, 3.0);

        let expected_point = Point::new(2.0, 4.0, 6.0);
        let point_multiplication = point_a * 2.0;
        assert_eq!(expected_point, point_multiplication);
    }

    #[test]
    fn point_division_scalar() {
        let point_a = Point::new(2.0, 2.0, 4.0);

        let expected_point = Point::new(1.0, 1.0, 2.0);
        let point_division = point_a / 2.0;
        assert_eq!(expected_point, point_division);
    }

    #[test]
    fn point_negative() {
        let point_a = Point::new(2.0, 2.0, 4.0);

        let expected_point = Point::new(-2.0, -2.0, -4.0);

        assert_eq!(expected_point, -point_a);
    }
}
