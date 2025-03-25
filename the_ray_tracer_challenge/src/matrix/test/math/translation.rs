#[cfg(test)]
mod matrix_translation_test {
    use crate::{assert_fuzzy_eq, matrix::model::Matrix, tuple::model::Tuple};

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let expected = Tuple::point(2.0, 1.0, 7.0);

        let result = transform * p;
        assert_fuzzy_eq!(result, expected);
    }
}
