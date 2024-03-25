const MATRIX_SIZE: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct Matrix2 {
    pub data: [[f64; MATRIX_SIZE]; MATRIX_SIZE],
}

impl Matrix2 {
    pub fn determinant(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}

impl std::ops::Index<usize> for Matrix2 {
    type Output = [f64; 2];

    fn index(&self, row: usize) -> &[f64; 2] {
        &self.data[row]
    }
}

impl std::ops::IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, row: usize) -> &mut [f64; 2] {
        &mut self.data[row]
    }
}

#[cfg(test)]
mod matrix2_tests {
    use super::*;

    #[test]
    fn matrix2_creation() {
        let matrix = Matrix2::from(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);

        assert_eq!(matrix.data[0][0], -3.0);
        assert_eq!(matrix.data[0][1], 5.0);
        assert_eq!(matrix.data[1][0], 1.0);
        assert_eq!(matrix.data[1][1], -2.0);
    }

    #[test]
    fn matrix2_equality() {
        let matrix_a = Matrix2::from(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);
        let matrix_b = matrix_a.clone();

        assert_eq!(matrix_a, matrix_b);
    }

    #[test]
    fn matrix2_determinant() {
        let matrix = Matrix2::new();
        let mut expected_result = 0.0;
        assert_eq!(expected_result, matrix.determinant());

        let matrix = Matrix2::from(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        expected_result = 17.0;
        assert_eq!(expected_result, matrix.determinant());
    }
}
