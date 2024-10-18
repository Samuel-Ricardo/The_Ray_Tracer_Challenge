use core::panic;

use crate::{
    matrix::{self, model::Matrix},
    tuple::model::Tuple,
    utils::equality::FuzzyEq,
};

impl Mul<Tuple> for Matrix<4> {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self[0][0] * other.x
                + self[0][1] * other.y
                + self[0][2] * other.z
                + self[0][3] * other.w,
            self[1][0] * other.x
                + self[1][1] * other.y
                + self[1][2] * other.z
                + self[1][3] * other.w,
            self[2][0] * other.x
                + self[2][1] * other.y
                + self[2][2] * other.z
                + self[2][3] * other.w,
            self[3][0] * other.x
                + self[3][1] * other.y
                + self[3][2] * other.z
                + self[3][3] * other.w,
        )
    }
}

impl Matrix<4> {
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix<3> {
        let mut matrix = Matrix::new();

        let mut source_row: usize = 0;
        let mut source_column: usize = 0;

        let mut target_row: usize = 0;
        let mut target_column: usize = 0;

        while target_row < 3 {
            if source_row == row {
                source_row += 1;
            }

            while target_column < 3 {
                if source_column == column {
                    source_column += 1;
                }

                matrix[target_row][target_column] = self[source_row][source_column];

                source_column += 1;
                target_column += 1;
            }

            source_row += 1;
            source_column = 0;

            target_row += 1;
            target_column = 0;
        }

        matrix
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);

        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut determinant = 0.0;

        for column in 0..4 {
            determinant += self.cofactor(0, column) * self[0][column];
        }

        return determinant;
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant().fuzzy_ne(0.0)
    }

    pub fn inverse(&self) -> Matrix<4> {
        if !self.is_invertible() {
            panic!("matrix is not invertible");
        }

        let mut matrix = Matrix::new();
        let determinant = self.determinant();

        for row in 0..4 {
            for column in 0..4 {
                let cofactor = self.cofactor(row, column);
                matrix[column][row] = cofactor / determinant;
            }
        }

        return matrix;
    }
}
