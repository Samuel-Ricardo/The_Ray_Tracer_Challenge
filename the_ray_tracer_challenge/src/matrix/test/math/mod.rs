#[cfg(test)]
mod tests {
    use crate::{assert_fuzzy_eq, matrix::model::Matrix, utils::equality::FuzzyEq};

    #[test]
    fn transpose_a_4x4_matrix() {
        let m = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let expected_result = Matrix::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        let actual_result = m.transpose();
        assert_eq!(actual_result.fuzzy_eq(expected_result), true);
    }

    #[test]
    fn determinant_of_a_2x2_matrix() {
        let m = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);

        let expected_result = 17.0;
        let actual_result = m.determinant();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn submatrix_of_a_3x3_matrix() {
        let m = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, 3.0]]);

        let expected_result = Matrix::from([[-3.0, 2.0], [0.0, 6.0]]);
        let actual_result = m.submatrix(0, 2);

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn submatrix_of_a_4x4_matrix() {
        let m = Matrix::from([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        let expected_result = Matrix::from([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

        let actual_result = m.submatrix(2, 1);

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn minor_of_a_3x3_matrix() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let sub = m.submatrix(1, 0);
        let determinant = sub.determinant();
        let minor = m.minor(1, 0);

        assert_fuzzy_eq!(25.0, determinant);
        assert_fuzzy_eq!(25.0, minor);
    }

    #[test]
    fn cofactor_of_a_3x3_matrix() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let first_minor_expected = m.minor(0, 0);
        let second_minor_expected = m.minor(1, 0);

        let first_cofactor_expected = m.cofactor(0, 0);
        let second_cofactor_expected = m.cofactor(1, 0);

        assert_fuzzy_eq!(-12.0, first_minor_expected);
        assert_fuzzy_eq!(-12.0, first_cofactor_expected);

        assert_fuzzy_eq!(25.0, second_minor_expected);
        assert_fuzzy_eq!(-25.0, second_cofactor_expected);
    }

    #[test]
    fn determinant_of_a_3x3_matrix() {
        let matrix = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        let cofactor0 = matrix.cofactor(0, 0);
        let cofactor1 = matrix.cofactor(0, 1);
        let cofactor2 = matrix.cofactor(0, 2);

        let determinant = matrix.determinant();

        assert_fuzzy_eq!(56.0, cofactor0);
        assert_fuzzy_eq!(12.0, cofactor1);
        assert_fuzzy_eq!(-46.0, cofactor2);

        assert_fuzzy_eq!(-196.0, determinant);
    }

    #[test]
    fn determinant_of_a_4x4_matrix() {
        let matrix = Matrix::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        let cofactor00 = matrix.cofactor(0, 0);
        let cofactor01 = matrix.cofactor(0, 1);
        let cofactor02 = matrix.cofactor(0, 2);
        let cofactor03 = matrix.cofactor(0, 3);

        let determinant = matrix.determinant();

        assert_fuzzy_eq!(690.0, cofactor00);
        assert_fuzzy_eq!(447.0, cofactor01);
        assert_fuzzy_eq!(210.0, cofactor02);
        assert_fuzzy_eq!(51.0, cofactor03);

        assert_fuzzy_eq!(-4071.0, determinant);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let matrix = Matrix::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        let determinant = matrix.determinant();

        assert_fuzzy_eq!(-2120.0, determinant);
        assert!(matrix.is_invertible());
    }

    #[test]
    fn testing_an_noninvertible_matrix_for_invertibility() {
        let m = Matrix::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        let determinant = m.determinant();

        assert_fuzzy_eq!(0.0, determinant);
        assert!(!m.is_invertible());
    }
}
