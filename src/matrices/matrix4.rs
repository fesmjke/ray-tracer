use crate::float_eq::ApproxEq;
use crate::matrices::{Matrix, Matrix3};
use crate::point::Point;
use crate::vector::Vector3;
use std::ops::Mul;

const MATRIX_SIZE: usize = 4;

#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    pub data: [f64; MATRIX_SIZE * MATRIX_SIZE],
}

impl Matrix4 {
    pub fn new() -> Self {
        Self {
            data: [0.0; MATRIX_SIZE * MATRIX_SIZE],
        }
    }

    pub fn from(vec: Vec<Vec<f64>>) -> Self {
        let mut temp = Self::new();

        for row in 0..MATRIX_SIZE {
            for col in 0..MATRIX_SIZE {
                temp[(row, col)] = vec[row][col];
            }
        }

        Self { data: temp.data }
    }

    pub fn determinant(&self) -> f64 {
        let mut res = 0.0;
        for col in 0..MATRIX_SIZE {
            res += self[(0, col)] * self.cofactor(0, col);
        }

        res
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix3 {
        let mut sb = vec![];

        for i in 0..self.rows() {
            for j in 0..self.columns() {
                if i == row || j == column {
                    continue;
                } else {
                    sb.push(self[(i, j)]);
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

    pub fn set(mut self, row: usize, column: usize, value: f64) -> Self {
        self[(row, column)] = value;
        self
    }

    pub fn invert(&self) -> Matrix4 {
        if self.is_invertible() {
            let dt = self.determinant();

            let mut res = Self::new();

            for row in 0..MATRIX_SIZE {
                for col in 0..MATRIX_SIZE {
                    let c = self.cofactor(row, col);
                    res[(col, row)] = c / dt;
                }
            }

            res
        } else {
            panic!("Matrix {:?} is not invertible!", &self);
        }
    }

    fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(&0.0)
    }
}

impl Matrix for Matrix4 {
    const COLUMNS: usize = MATRIX_SIZE;
    const ROWS: usize = MATRIX_SIZE;

    fn columns(&self) -> usize {
        MATRIX_SIZE
    }

    fn rows(&self) -> usize {
        MATRIX_SIZE
    }

    fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    fn transpose(&mut self) -> Self {
        let mut res = Self::new();

        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                res[(j, i)] = self[(i, j)];
            }
        }

        res
    }
}

impl std::ops::Index<(usize, usize)> for Matrix4 {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);
        unsafe { self.data.get_unchecked(row * MATRIX_SIZE + col) }
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        debug_assert!(row < MATRIX_SIZE);
        debug_assert!(col < MATRIX_SIZE);
        unsafe { self.data.get_unchecked_mut(row * MATRIX_SIZE + col) }
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                if !self[(i, j)].approx_eq_low(&other[(i, j)]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut res = Self::new();

        for row in 0..MATRIX_SIZE {
            for col in 0..MATRIX_SIZE {
                res[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
                    + self[(row, 3)] * rhs[(3, col)];
            }
        }

        res
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        // horrible solution
        Self::Output::new(
            self[(0, 0)] * other.x + self[(0, 1)] * other.y + self[(0, 2)] * other.z,
            self[(1, 0)] * other.x + self[(1, 1)] * other.y + self[(1, 2)] * other.z,
            self[(2, 0)] * other.x + self[(2, 1)] * other.y + self[(2, 2)] * other.z,
        )
    }
}

impl Mul<Point> for Matrix4 {
    type Output = Point;

    fn mul(self, other: Point) -> Self::Output {
        // horrible solution
        Self::Output::new(
            self[(0, 0)] * other.x + self[(0, 1)] * other.y + self[(0, 2)] * other.z + self[(0, 3)],
            self[(1, 0)] * other.x + self[(1, 1)] * other.y + self[(1, 2)] * other.z + self[(1, 3)],
            self[(2, 0)] * other.x + self[(2, 1)] * other.y + self[(2, 2)] * other.z + self[(2, 3)],
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

        assert_eq!(matrix[(0, 0)], -3.0);
        assert_eq!(matrix[(0, 2)], 1.0);
        assert_eq!(matrix[(0, 3)], 3.0);
        assert_eq!(matrix[(1, 1)], -2.0);
        assert_eq!(matrix[(2, 2)], 1.0);
        assert_eq!(matrix[(3, 0)], 2.0);
        assert_eq!(matrix[(3, 3)], -1.0);
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
        let expected_result = matrix[(0,0)] * cofactor_a // -4071.0
            + matrix[(0,1)] * cofactor_b
            + matrix[(0,2)] * cofactor_c
            + matrix[(0,3)] * cofactor_d;

        assert_eq!(expected_result, dt);
        assert_eq!(expected_result_explicit, dt);
        assert_eq!(expected_result_explicit, expected_result);
    }

    #[test]
    fn matrix4_is_invertible() {
        let matrix_a = Matrix4::from(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);

        let matrix_b = Matrix4::from(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);

        assert!(matrix_a.is_invertible()); // yes -> determinant = -2120
        assert!(!matrix_b.is_invertible()); // no -> determinant = 0
    }

    #[test]
    fn matrix_inverse() {
        let matrix_a = Matrix4::from(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);

        let matrix_a_inverted = matrix_a.invert();

        let expected_matrix = Matrix4::from(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_eq!(expected_matrix, matrix_a_inverted);

        let matrix_b = Matrix4::from(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);

        let matrix_b_inverted = matrix_b.invert();

        let expected_matrix = Matrix4::from(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(expected_matrix, matrix_b_inverted);

        let matrix_c = Matrix4::from(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);

        let matrix_c_inverted = matrix_c.invert();

        let expected_matrix = Matrix4::from(vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(expected_matrix, matrix_c_inverted);
    }

    #[test]
    fn matrix4_back_initial() {
        let matrix_a = Matrix4::from(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ]);

        let matrix_b = Matrix4::from(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ]);

        let matrix_c = matrix_a * matrix_b;
        let initial_matrix = matrix_c * matrix_b.invert();
        assert_eq!(initial_matrix, matrix_a)
    }

    #[test]
    fn matrix4_chain_set() {
        let matrix_a = Matrix4::identity().set(0, 3, 1.0);

        let expected_matrix = Matrix4::from(vec![
            vec![1.0, 0.0, 0.0, 1.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(expected_matrix, matrix_a)
    }
}
