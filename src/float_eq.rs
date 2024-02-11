pub const EPSILON: f64 = 1.0e-7;

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(&self, other: &Rhs) -> bool;
}
