use crate::matrix::model::Matrix;

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
}
