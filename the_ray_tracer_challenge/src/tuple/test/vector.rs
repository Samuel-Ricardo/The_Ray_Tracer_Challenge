#[cfg(test)]
mod tests {
    use crate::tuple::{model::Tuple, test};

    #[test]
    fn vector_initialize_correctly() {
        let v = Tuple::Vector(1.0, 2.0, 3.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);

        assert_eq!(v.w, 0.0);
    }

    #[test]
    fn is_vector_verifier() {
        let v = Tuple::Vector(1.0, 2.0, 3.0);

        assert_eq!(v.is_vector(), true);
    }

    #[test]
    fn not_is_vector_verifier() {
        let v = Tuple::Point(1.0, 2.0, 3.0);

        assert_eq!(v.is_vector(), false);
    }

    #[test]
    fn sub_of_two_vectors() {
        let t1 = Tuple::Vector(3.0, -2.0, 5.0);
        let t2 = Tuple::Vector(-2.0, 3.0, 1.0);

        let expected = Tuple::Vector(5.0, -5.0, 4.0);
        let result = t1 - t2;

        assert_eq!(result, expected);
        assert!(result.is_vector());
    }

    #[test]
    fn compute_magnitude_of_vector_1_0_0() {
        let v = Tuple::Vector(1.0, 0.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn compute_magnitude_of_vector_0_1_0() {
        let v = Tuple::Vector(0.0, 1.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn compute_magnitude_of_vector_0_0_1() {
        let v = Tuple::Vector(0.0, 0.0, 1.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn compute_magnitude_of_vector_1_2_3() {
        let v = Tuple::Vector(1.0, 2.0, 3.0);
        let expected = 14.0_f64.sqrt();

        assert_eq!(v.magnitude(), expected);
    }

    #[test]
    fn compute_magnitude_of_negative_vector_1_2_3() {
        let v = Tuple::Vector(-1.0, -2.0, -3.0);
        let expected = 14.0_f64.sqrt();

        assert_eq!(v.magnitude(), expected);
    }

    #[test]
    fn normalize_vector_4_0_0() {
        let v = Tuple::Vector(4.0, 0.0, 0.0);
        let expected = Tuple::Vector(1.0, 0.0, 0.0);

        assert_eq!(v.normalize(), expected);
    }

    #[test]
    fn normalize_vector_1_2_3() {
        let vector = Tuple::Vector(1.0, 2.0, 3.0);
        let expected = Tuple::Vector(0.26726, 0.53452, 0.80178);

        assert_eq!(vector.normalize(), expected);
    }
}
