use super::model::Matrix;

pub mod x2;
pub mod x3;
pub mod x4;

impl<const D: usize> Matrix<D> {
    pub fn transpose(&self) -> Matrix<D> {
        let mut matrix = Matrix::new();

        for row in 0..D {
            for column in 0..D {
                matrix[column][row] = self[row][column];
            }
        }

        matrix
    }
}
