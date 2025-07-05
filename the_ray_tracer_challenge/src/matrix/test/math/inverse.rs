#[cfg(test)]
mod matrix_shearing_test {
    use crate::{
        assert_fuzzy_eq, matrix::model::Matrix, tuple::model::Tuple, utils::equality::FuzzyEq,
    };

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

    #[test]
    fn calculating_the_inverse_of_a_4x4_matrix() {
        let m = Matrix::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let determinant = m.determinant();
        let cofactor23 = m.cofactor(2, 3);
        let cofactor32 = m.cofactor(3, 2);

        let expected_result = Matrix::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        let actual_result = m.inverse();

        assert_fuzzy_eq!(532.0, determinant);
        assert_fuzzy_eq!(-160.0, cofactor23);
        assert_fuzzy_eq!(-160.0 / 532.0, actual_result[3][2]);
        assert_fuzzy_eq!(105.0, cofactor32);
        assert_fuzzy_eq!(105.0 / 532.0, actual_result[2][3]);
        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn calculating_the_inverse_of_another_4x4_matrix() {
        let m = Matrix::from([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        let expected_result = Matrix::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        let actual_result = m.inverse();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn calculating_the_inverse_of_a_third_4x4_matrix() {
        let m = Matrix::from([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        let expected_result = Matrix::from([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        let actual_result = m.inverse();

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let m1 = Matrix::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let m2 = Matrix::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let m3 = m1 * m2;

        let actual_result = m3 * m2.inverse();

        assert_fuzzy_eq!(actual_result, m1);
    }
}
