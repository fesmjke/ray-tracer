use crate::float_eq::{ApproxEq, EPSILON};
use std::ops::{Add, Div, Mul, Sub};

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
