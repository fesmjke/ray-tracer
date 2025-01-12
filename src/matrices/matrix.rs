use crate::float_eq::ApproxEq;
use crate::matrices::{Matrix2, Matrix3};
use std::ops::Mul;

pub trait Matrix {
    const COLUMNS: usize;
    const ROWS: usize;

    fn columns(&self) -> usize;
    fn rows(&self) -> usize;
    fn identity() -> Self;
    fn transpose(&mut self) -> Self;
}

macro_rules! impl_matrix {
    ($ty: ty,$col:expr, $row:expr) => {
        impl Matrix for $ty {
            const COLUMNS: usize = $col;
            const ROWS: usize = $row;

            fn columns(&self) -> usize {
                $col
            }

            fn rows(&self) -> usize {
                $row
            }

            fn identity() -> Self {
                let mut v = Self::new().data;

                for i in 0..$row {
                    for j in 0..$col {
                        if i == j {
                            v[i][j] = 1.0;
                        }
                    }
                }

                Self { data: v }
            }

            fn transpose(&mut self) -> Self {
                let temp = self.data.clone();

                for (i, iv) in temp.iter().enumerate() {
                    for (j, _) in iv.iter().enumerate() {
                        self.data[i][j] = temp[j][i];
                    }
                }

                self.clone()
            }
        }

        impl $ty {
            pub fn new() -> Self {
                Self {
                    data: [[0.0; $col]; $row],
                }
            }

            pub fn from(data: Vec<Vec<f64>>) -> Self {
                let mut temp = Self::new().data;

                for row in 0..$row {
                    for col in 0..$col {
                        temp[row][col] = data[row][col];
                    }
                }

                Self { data: temp }
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                Self::new()
            }
        }

        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                for i in 0..self.rows() {
                    for j in 0..self.columns() {
                        if !self.data[i][j].approx_eq_low(&other.data[i][j]) {
                            return false;
                        }
                    }
                }

                true
            }
        }

        impl Mul for $ty {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                let mut result = Self::new();

                for row in 0..self.rows() {
                    for col in 0..self.columns() {
                        let mut delta = 0.0;
                        for i in 0..self.rows() {
                            delta += self.data[row][i] * other.data[i][col];
                        }
                        result.data[row][col] = delta;
                    }
                }

                result
            }
        }
    };
}

impl_matrix!(Matrix2, 2, 2);
impl_matrix!(Matrix3, 3, 3);

#[cfg(test)]
mod matrix_tests {
    use super::*;
    use crate::matrices::Matrix4;
    use crate::point::Point;
    use crate::vector::Vector3;

    #[test]
    fn matrix_creation() {
        let mut matrix = Matrix4::default();

        matrix[(0, 0)] = 1.0;
        matrix[(0, 1)] = 2.0;
        matrix[(0, 2)] = 3.0;
        matrix[(0, 3)] = 4.0;

        matrix[(1, 0)] = 5.5;
        matrix[(1, 1)] = 6.5;
        matrix[(1, 2)] = 7.5;
        matrix[(1, 3)] = 8.5;

        matrix[(2, 0)] = 9.0;
        matrix[(2, 1)] = 10.0;
        matrix[(2, 2)] = 11.0;
        matrix[(2, 3)] = 12.0;

        matrix[(3, 0)] = 13.5;
        matrix[(3, 1)] = 14.5;
        matrix[(3, 2)] = 15.5;
        matrix[(3, 3)] = 16.5;

        assert_eq!(matrix[(3, 0)], 13.5);
        assert_eq!(matrix[(1, 2)], 7.5);
    }

    #[test]
    fn matrix_identity() {
        let matrix = Matrix4::identity();

        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(1, 1)], 1.0);
        assert_eq!(matrix[(2, 2)], 1.0);
        assert_eq!(matrix[(3, 3)], 1.0);

        assert_eq!(matrix[(0, 1)], 0.0);
    }

    #[test]
    fn matrix_identity_by_vector() {
        let identity = Matrix4::identity();
        let vector_b = Vector3::new(1.0, 2.0, 3.0);

        let expected_vector = Vector3::new(1.0, 2.0, 3.0);
        let multiplication = identity * vector_b;

        assert_eq!(expected_vector, multiplication);
    }

    #[test]
    fn matrix_identity_by_matrix() {
        let matrix_a = Matrix4::from(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);

        let expected_matrix = Matrix4::from(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);

        let multiplication = matrix_a * Matrix4::identity();

        assert_eq!(expected_matrix, multiplication);
    }

    #[test]
    fn matrix_equality() {
        let mut matrix_a = Matrix2::default();

        matrix_a.data[0][0] = 1.0;
        matrix_a.data[0][1] = 2.0;
        matrix_a.data[1][0] = 3.0;
        matrix_a.data[1][1] = 4.0;

        let mut matrix_b = Matrix2::default();

        matrix_b.data[0][0] = 1.0;
        matrix_b.data[0][1] = 2.0;
        matrix_b.data[1][0] = 3.0;
        matrix_b.data[1][1] = 4.0;

        assert_eq!(matrix_a, matrix_b);

        matrix_a.data[0][0] = 0.0;

        assert_ne!(matrix_a, matrix_b);
    }

    #[test]
    fn matrix_transpose() {
        let identity = Matrix4::identity().transpose();
        let expected_identity = Matrix4::identity();

        assert_eq!(expected_identity, identity);

        let mut matrix_a = Matrix2::from(vec![vec![2.0, 1.0], vec![3.0, 1.0]]);
        let expected_matrix = Matrix2::from(vec![vec![2.0, 3.0], vec![1.0, 1.0]]);

        matrix_a.transpose();

        assert_eq!(expected_matrix, matrix_a);

        let matrix_b = Matrix4::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![4.0, 2.0, 3.0, 4.0],
            vec![6.0, 2.0, 3.0, 4.0],
            vec![8.0, 2.0, 3.0, 4.0],
        ])
        .transpose();

        let expected_matrix = Matrix4::from(vec![
            vec![1.0, 4.0, 6.0, 8.0],
            vec![2.0, 2.0, 2.0, 2.0],
            vec![3.0, 3.0, 3.0, 3.0],
            vec![4.0, 4.0, 4.0, 4.0],
        ]);

        assert_eq!(expected_matrix, matrix_b);
    }

    #[test]
    fn matrix_transpose_identity() {
        let mut identity = Matrix4::identity();

        identity.transpose();

        assert_eq!(identity, Matrix4::identity());
    }

    #[test]
    fn matrix_multiplication() {
        let matrix_a = Matrix4::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix_b = Matrix4::from(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        let expected_matrix = Matrix4::from(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);

        let multiplication = matrix_a * matrix_b;

        assert_eq!(expected_matrix, multiplication);
    }

    #[test]
    fn matrix_multiplication_by_vector() {
        let matrix_a = Matrix4::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        let vector_b = Vector3::new(1.0, 2.0, 3.0);

        let expected_vector = Vector3::new(14.0, 22.0, 32.0);

        let multiplication = matrix_a * vector_b;

        assert_eq!(expected_vector, multiplication);
    }

    #[test]
    fn matrix_multiplication_point() {
        let matrix_a = Matrix4::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        let point = matrix_a * Point::new(1.0, 2.0, 3.0);
        let expected_point = Point::new(18.0, 24.0, 33.0);

        assert_eq!(expected_point, point);
    }
}
