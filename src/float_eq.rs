pub const EPSILON: f64 = 1.0e-7;
pub const LOW_EPSILON: f64 = 1.0e-4;

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(&self, other: &Rhs) -> bool;
    fn approx_eq_low(&self, other: &Rhs) -> bool {
        self.approx_eq(other)
    }
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        (self - other).abs() < EPSILON
    }

    fn approx_eq_low(&self, other: &Self) -> bool {
        (self - other).abs() < LOW_EPSILON
    }
}
