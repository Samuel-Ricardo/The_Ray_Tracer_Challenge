#[cfg(test)]
mod tests {
    use crate::{
        assert_fuzzy_eq, assert_fuzzy_ne, matrix::model::Matrix, utils::equality::FuzzyEq,
    };

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
}
