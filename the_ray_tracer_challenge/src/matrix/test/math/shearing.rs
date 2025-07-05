#[cfg(test)]
mod matrix_shearing_test {
    use crate::{
        assert_fuzzy_eq, matrix::model::Matrix, tuple::model::Tuple, utils::equality::FuzzyEq,
    };

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected_result = Tuple::point(5.0, 3.0, 4.0);
        let actual_result = transform * point;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected_result = Tuple::point(6.0, 3.0, 4.0);
        let actual_result = transform * point;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected_result = Tuple::point(2.0, 5.0, 4.0);
        let actual_result = transform * point;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected_result = Tuple::point(2.0, 7.0, 4.0);
        let actual_result = transform * point;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected_result = Tuple::point(2.0, 3.0, 6.0);
        let actual_result = transform * point;

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Tuple::point(2.0, 3.0, 4.0);

        let expected_result = Tuple::point(2.0, 3.0, 7.0);
        let actual_result = transform * point;

        assert_fuzzy_eq!(actual_result, expected_result);
    }
}
