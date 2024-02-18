pub const EPSILON: f64 = 1.0e-7;

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(&self, other: &Rhs) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        (self - other).abs() < EPSILON
    }
}
