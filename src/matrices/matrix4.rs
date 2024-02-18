use crate::vector::Vector3;
use std::ops::Mul;
pub struct Matrix4 {
    pub data: Vec<Vec<f64>>,
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        // horrible solution
        Self::Output::new(
            self.data[0][0] * other.x
                + self.data[0][1] * other.y
                + self.data[0][2] * other.z
                + self.data[0][3],
            self.data[1][0] * other.x
                + self.data[1][1] * other.y
                + self.data[1][2] * other.z
                + self.data[1][3],
            self.data[2][0] * other.x
                + self.data[2][1] * other.y
                + self.data[2][2] * other.z
                + self.data[2][3],
        )
    }
}
