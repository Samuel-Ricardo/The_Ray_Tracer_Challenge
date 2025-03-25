#[cfg(test)]
mod matrix_translation_test {
    use crate::{matrix::model::Matrix, tuple::model::Tuple};

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let expected_result = Tuple::point(2.0, 1.0, 7.0);

        let actual_result = transform * p;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inverse_transform = transform.inverse();

        let p = Tuple::point(-3.0, 4.0, 5.0);
        let expected_result = Tuple::point(-8.0, 7.0, 3.0);

        let actual_result = inverse_transform * p;
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        let expected_result = v;
        let actual_result = transform * v;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        let expected_result = Tuple::point(-8.0, 18.0, 32.0);
        let actual_result = transform * p;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        let expected_result = Tuple::vector(-8.0, 18.0, 32.0);
        let actual_result = transform * v;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inverse_transform = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        let expected_result = Tuple::vector(-2.0, 2.0, 2.0);
        let actual_result = inverse_transform * v;

        assert_eq!(actual_result, expected_result);
    }
}
