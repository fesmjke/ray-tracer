use crate::matrices::{Matrix, Matrix2};

#[derive(Clone)]
pub struct Matrix3 {
    pub data: Vec<Vec<f64>>,
}

impl Matrix3 {
    pub fn determinant(&self) -> f64 {
        todo!()
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
    fn matrix3_determinant() {
        todo!()
    }
}
