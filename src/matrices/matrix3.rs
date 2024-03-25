use crate::matrices::{Matrix, Matrix2};

const MATRIX_SIZE: usize = 3;

#[derive(Copy, Clone, Debug)]
pub struct Matrix3 {
    pub data: [[f64; MATRIX_SIZE]; MATRIX_SIZE],
}

impl Matrix3 {
    pub fn determinant(&self) -> f64 {
        self.data[0]
            .iter()
            .enumerate()
            .fold(0.0, |acc, (i, x)| acc + (x * self.cofactor(0, i)))
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix2 {
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
            .chunks(Matrix2::ROWS)
            .map(|chunk| chunk.into())
            .collect::<Vec<Vec<f64>>>();

        Matrix2::from(sb)
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

impl std::ops::Index<usize> for Matrix3 {
    type Output = [f64; 3];

    fn index(&self, row: usize) -> &[f64; 3] {
        &self.data[row]
    }
}

impl std::ops::IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, row: usize) -> &mut [f64; 3] {
        &mut self.data[row]
    }
}

#[cfg(test)]
mod matrix3_tests {
    use super::*;

    #[test]
    fn matrix3_creation() {
        let matrix = Matrix3::from(vec![
            vec![-3.0, 5.0, 1.0],
            vec![1.0, -2.0, 3.0],
            vec![4.0, -6.0, 1.0],
        ]);

        assert_eq!(matrix.data[0][0], -3.0);
        assert_eq!(matrix.data[0][2], 1.0);
        assert_eq!(matrix.data[1][1], -2.0);
        assert_eq!(matrix.data[1][1], -2.0);
        assert_eq!(matrix.data[2][0], 4.0);
        assert_eq!(matrix.data[2][2], 1.0);
    }

    #[test]
    fn matrix3_equality() {
        let matrix_a = Matrix3::from(vec![
            vec![-3.0, 5.0, 1.0],
            vec![1.0, -2.0, 3.0],
            vec![4.0, -6.0, 1.0],
        ]);
        let matrix_b = matrix_a.clone();

        assert_eq!(matrix_a, matrix_b);
    }

    #[test]
    fn matrix3_submatrix() {
        let matrix_a = Matrix3::from(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);

        let sub_a = matrix_a.submatrix(0, 0);
        let expected_matrix = Matrix2::from(vec![vec![2.0, 7.0], vec![6.0, -3.0]]);

        assert_eq!(expected_matrix, sub_a);

        let sub_a = matrix_a.submatrix(1, 0);
        let expected_matrix = Matrix2::from(vec![vec![5.0, 0.0], vec![6.0, -3.0]]);

        assert_eq!(expected_matrix, sub_a);

        let sub_a = matrix_a.submatrix(2, 1);
        let expected_matrix = Matrix2::from(vec![vec![1.0, 0.0], vec![-3.0, 7.0]]);

        assert_eq!(expected_matrix, sub_a);

        let sub_a = matrix_a.submatrix(0, 2);
        let expected_matrix = Matrix2::from(vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);

        assert_eq!(expected_matrix, sub_a);
    }

    #[test]
    fn matrix3_minor() {
        let matrix = Matrix3::from(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        let dt = matrix.minor(1, 0);
        let expected_dt = 25.0;

        assert_eq!(expected_dt, dt);
    }

    #[test]
    fn matrix3_cofactor() {
        let matrix = Matrix3::from(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        let dt = matrix.minor(0, 0);
        let cf = matrix.cofactor(0, 0);
        assert_eq!(dt, cf);

        let dt = matrix.minor(1, 0);
        let cf = matrix.cofactor(1, 0);
        assert_ne!(dt, cf);
    }

    #[test]
    fn matrix3_determinant() {
        let matrix = Matrix3::from(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);

        let cofactor_a = matrix.cofactor(0, 0);
        let cofactor_b = matrix.cofactor(0, 1);
        let cofactor_c = matrix.cofactor(0, 2);

        let dt = matrix.determinant();
        let expected_result = matrix.data[0][0] * cofactor_a // -196.0
            + matrix.data[0][1] * cofactor_b
            + matrix.data[0][2] * cofactor_c;

        assert_eq!(expected_result, dt);
    }
}
