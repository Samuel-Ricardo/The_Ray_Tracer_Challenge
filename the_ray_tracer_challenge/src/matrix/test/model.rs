#[cfg(test)]
mod tests {
    use crate::{assert_fuzzy_eq, matrix::model::Matrix, utils::equality::FuzzyEq};

    #[test]
    fn create_a_valid_2x2_matrix() {
        let m = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn create_a_valid_3x3_matrix() {
        let m = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[0][2], 0.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[1][2], -7.0);
        assert_eq!(m[2][0], 0.0);
        assert_eq!(m[2][1], 1.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    fn create_a_valid_4x4_matrix() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[0][2], 3.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[1][3], 8.5);
        assert_eq!(m[2][0], 9.0);
        assert_eq!(m[2][1], 10.0);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[2][3], 12.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][1], 14.5);
        assert_eq!(m[3][2], 15.5);
        assert_eq!(m[3][3], 16.5);
    }

    #[test]
    fn matrix_equality_with_identical_2x2_matrices() {
        let m1 = Matrix::from([[0.123456789, 1.0], [2.0, 3.0]]);
        let m2 = Matrix::from([[0.123456789, 1.0], [2.0, 3.0]]);

        assert_fuzzy_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_almost_identical_2x2_matrices() {
        let m1 = Matrix::from([[0.123456789, 1.0], [2.0, 3.0]]);
        let m2 = Matrix::from([[0.123456780, 1.0], [2.0, 3.0]]);

        assert_fuzzy_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_identical_3x3_matrices() {
        let m1 = Matrix::from([
            [0.123456789, 1.0, 2.0],
            [2.0, 3.0, 4.0],
            [5.0, 6.0, 7.7777777777777777],
        ]);

        let m2 = Matrix::from([
            [0.123456789, 1.0, 2.0],
            [2.0, 3.0, 4.0],
            [5.0, 6.0, 7.7777777777777777],
        ]);

        assert_fuzzy_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_almost_identical_3x3_matrices() {
        let m1 = Matrix::from([
            [0.123456789, 1.0, 2.0],
            [2.0, 3.0, 4.0],
            [5.0, 6.0, 7.7777777777777777],
        ]);
        let m2 = Matrix::from([
            [0.123456780, 1.0, 2.0],
            [2.0, 3.0, 4.0],
            [5.0, 6.0, 7.7777777777777],
        ]);

        assert_fuzzy_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_identical_4x4_matrices() {
        let m1 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let m2 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_fuzzy_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_almost_identical_4x4_matrices() {
        let m1 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0000000000001],
        ]);
        let m2 = Matrix::from([
            [0.123456789, 1.0, 2.0, 42.0],
            [2.0, 3.0, 4.0, -42.0],
            [5.0, 6.0, 7.7777777777777, 23.5],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_fuzzy_eq!(m1, m2);
    }
}
