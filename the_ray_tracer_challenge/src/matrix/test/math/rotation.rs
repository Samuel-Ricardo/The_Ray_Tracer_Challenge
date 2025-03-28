#[cfg(test)]
mod matrix_scaling_test {
    use std::f64::consts::PI;

    use crate::{matrix::model::Matrix, tuple::model::Tuple};

    #[test]
    fn rotating_a_point_around_the_x_axis_half_quarter() {
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let p = Tuple::point(0.0, 1.0, 0.0);

        let expected_result = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
        let actual_result = half_quarter * p;

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis_full_quarter() {
        let full_quarter = Matrix::rotation_x(PI / 2.0);
        let p = Tuple::point(0.0, 1.0, 0.0);

        let expected_result = Tuple::point(0.0, 0.0, 1.0);
        let actual_result = full_quarter * p;

        assert_eq!(actual_result, expected_result);
    }
}
