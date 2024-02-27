use crate::matrices::{Matrix, Matrix3};
use crate::vector::Vector3;
use std::ops::Mul;

#[derive(Clone)]
pub struct Matrix4 {
    pub data: Vec<Vec<f64>>,
}

impl Matrix4 {
    pub fn determinant(&self) -> f64 {
        self.data[0]
            .iter()
            .enumerate()
            .fold(0.0, |acc, (i, x)| acc + (x * self.cofactor(0, i)))
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix3 {
        let mut sb = vec![];

        for i in 0..self.rows() {
            for j in 0..self.columns() {
                if i == row || j == column {
                    continue;
                } else {
                    sb.push(self.data[i][j]);
                }
            }
        }

        let sb = sb
            .chunks(Matrix3::ROWS)
            .map(|chunk| chunk.into())
            .collect::<Vec<Vec<f64>>>();

        Matrix3::from(sb)
    }

    fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);

        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
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

#[cfg(test)]
mod matrix4_tests {
    use super::*;

    #[test]
    fn matrix4_creation() {
        let matrix = Matrix4::from(vec![
            vec![-3.0, 5.0, 1.0, 3.0],
            vec![1.0, -2.0, 3.0, 1.0],
            vec![4.0, -6.0, 1.0, 2.0],
            vec![2.0, -4.0, 4.0, -1.0],
        ]);

        assert_eq!(matrix.data[0][0], -3.0);
        assert_eq!(matrix.data[0][2], 1.0);
        assert_eq!(matrix.data[0][3], 3.0);
        assert_eq!(matrix.data[1][1], -2.0);
        assert_eq!(matrix.data[2][2], 1.0);
        assert_eq!(matrix.data[3][0], 2.0);
        assert_eq!(matrix.data[3][3], -1.0);
    }

    #[test]
    fn matrix4_equality() {
        let matrix_a = Matrix4::from(vec![
            vec![-3.0, 5.0, 1.0, 3.0],
            vec![1.0, -2.0, 3.0, 1.0],
            vec![4.0, -6.0, 1.0, 2.0],
            vec![2.0, -4.0, 4.0, -1.0],
        ]);
        let matrix_b = matrix_a.clone();

        assert_eq!(matrix_a, matrix_b);
    }

    #[test]
    fn matrix4_submatrix() {
        let matrix_a = Matrix4::from(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);

        let sub_a = matrix_a.submatrix(2, 1);
        let expected_matrix = Matrix3::from(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);

        assert_eq!(expected_matrix, sub_a);

        let sub_a = matrix_a.submatrix(0, 0);
        let expected_matrix = Matrix3::from(vec![
            vec![5.0, 8.0, 6.0],
            vec![0.0, 8.0, 2.0],
            vec![1.0, -1.0, 1.0],
        ]);

        assert_eq!(expected_matrix, sub_a);
    }

    #[test]
    fn matrix4_determinant() {
        let matrix = Matrix4::from(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);

        let cofactor_a = matrix.cofactor(0, 0);
        let cofactor_b = matrix.cofactor(0, 1);
        let cofactor_c = matrix.cofactor(0, 2);
        let cofactor_d = matrix.cofactor(0, 3);

        let dt = matrix.determinant();
        let expected_result_explicit = -4071.0;
        let expected_result = matrix.data[0][0] * cofactor_a // -4071.0
            + matrix.data[0][1] * cofactor_b
            + matrix.data[0][2] * cofactor_c
            + matrix.data[0][3] * cofactor_d;

        assert_eq!(expected_result, dt);
        assert_eq!(expected_result_explicit, dt);
        assert_eq!(expected_result_explicit, expected_result);
    }
}
