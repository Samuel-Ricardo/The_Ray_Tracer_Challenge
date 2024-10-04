use super::model::Matrix;

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

impl Matrix<2> {
    pub fn determinant(&self) -> f64 {
        (self[0][0] * self[1][1]) - (self[0][1] * self[1][0])
    }
}

impl Matrix<3> {
    pub fn submatrix(&self, row: usize, column: usize) -> Matrix<2> {
        let mut matrix: Matrix<2> = Matrix::new();

        let mut source_row = 0;
        let mut source_column = 0;

        let mut target_row = 0;
        let mut target_column = 0;

        while target_row < 2 {
            if source_row == row {
                source_row += 1;
            }

            while target_column < 2 {
                if source_row == row || source_column == column {
                    source_column += 1;
                    continue;
                }

                matrix[target_row][target_column] = self[source_row][source_column];

                source_column += 1;
                target_column += 1;
            }

            source_row += 1;
            target_row += 1;

            target_column = 0;
            source_column = 0;
        }

        return matrix;
    }
}
