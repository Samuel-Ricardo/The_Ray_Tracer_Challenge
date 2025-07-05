#[cfg(test)]
mod matrix_shearing_test {
    use std::f64::consts::PI;

    use png::Transformations;

    use crate::{
        assert_fuzzy_eq, matrix::model::Matrix, tuple::model::Tuple, utils::equality::FuzzyEq,
    };

    #[test]
    fn individual_transformation_are_applied_in_sequence() {
        let point = Tuple::point(1.0, 0.0, 1.0);
        let rotation_x = Matrix::rotation_x(PI / 2.0);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let translation = Matrix::translation(10.0, 5.0, 7.0);

        let rotated_point = rotation_x * point;
        assert_eq!(rotated_point, Tuple::point(1.0, -1.0, 0.0));

        let scaled_point = scaling * rotated_point;
        assert_eq!(scaled_point, Tuple::point(5.0, -5.0, 0.0));

        let translated_point = translation * scaled_point;
        assert_eq!(translated_point, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let point = Tuple::point(1.0, 0.0, 1.0);
        let rotation_x = Matrix::rotation_x(PI / 2.0);
        let scaling = Matrix::scaling(5.0, 5.0, 5.0);
        let translation = Matrix::translation(10.0, 5.0, 7.0);

        let transform = translation * scaling * rotation_x;

        let actual_result = transform * point;
        let expected_result = Tuple::point(15.0, 0.0, 7.0);

        assert_fuzzy_eq!(actual_result, expected_result);
    }
}
