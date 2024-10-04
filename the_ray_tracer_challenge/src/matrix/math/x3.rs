use crate::matrix::model::Matrix;

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

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }
}
