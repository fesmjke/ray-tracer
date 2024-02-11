use crate::float_eq::{ApproxEq, EPSILON};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(&other)
    }
}

impl ApproxEq for Color {
    fn approx_eq(&self, other: &Self) -> bool {
        (self.r - other.r).abs() < EPSILON
            && (self.g - other.g).abs() < EPSILON
            && (self.b - other.b).abs() < EPSILON
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Self) -> Self::Output {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Self) -> Self::Output {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Self) -> Self::Output {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Self::Output {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        self * other
    }
}

impl Div for Color {
    type Output = Color;
    fn div(self, other: Self) -> Self::Output {
        Color {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, other: f64) -> Self::Output {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;

    #[test]
    fn color_creation() {
        let df_color = Color::default();
        let custom_color = Color::new(0.9, 0.6, 0.75);

        assert_eq!(df_color.g, 0.0);
        assert_eq!(custom_color.b, 0.75);
    }

    #[test]
    fn color_addition() {
        let color_a = Color::new(0.1, 0.4, 0.2);
        let color_b = Color::new(0.1, 0.1, 0.2);

        let color_addition = color_a + color_b;
        assert_eq!(color_addition, Color::new(0.2, 0.5, 0.4));

        let color_addition = color_b + color_a;
        assert_eq!(color_addition, Color::new(0.2, 0.5, 0.4));

        let color_addition = color_a + color_a;
        assert_eq!(color_addition, Color::new(0.2, 0.8, 0.4));
    }

    #[test]
    fn color_subtraction() {
        let color_a = Color::new(0.1, 0.4, 0.2);
        let color_b = Color::new(0.1, 0.1, 0.2);

        let expected_color = Color::new(0.0, 0.3, 0.0);
        let color_subtraction = color_a - color_b;
        assert_eq!(expected_color, color_subtraction);

        let color_subtraction = color_b - color_a;
        let expected_color = Color::new(0.0, -0.3, 0.0);
        assert_eq!(expected_color, color_subtraction);

        let color_subtraction = color_a - color_a;
        let expected_color = Color::new(0.0, 0.0, 0.0);
        assert_eq!(expected_color, color_subtraction);
    }

    #[test]
    fn color_multiplication() {
        let color_a = Color::new(0.3, 0.0, 1.0);
        let color_b = Color::new(0.2, 0.0, 1.0);

        let expected_color = Color::new(0.6, 0.0, 1.0);
        let color_multiplication = color_a * color_b;
        assert_eq!(expected_color, color_multiplication);
    }

    #[test]
    fn color_multiplication_scalar() {
        let color_a = Color::new(0.35, 0.0, 0.45);

        let expected_color = Color::new(0.7, 0.0, 0.9);
        let color_multiplication = color_a * 2.0;
        assert_eq!(expected_color, color_multiplication);
    }

    #[test]
    fn color_division() {
        let color_a = Color::new(0.1, 0.1, 0.1);
        let color_b = Color::new(0.1, 0.1, 0.1);

        let expected_color = Color::new(1.0, 1.0, 1.0);
        let color_division = color_a / color_b;
        assert_eq!(expected_color, color_division);
    }
    #[test]
    fn color_division_scalar() {
        let color_a = Color::new(1.0, 1.0, 1.0);

        let expected_color = Color::new(0.5, 0.5, 0.5);
        let color_division = color_a / 2.0;
        assert_eq!(expected_color, color_division);
    }
}
