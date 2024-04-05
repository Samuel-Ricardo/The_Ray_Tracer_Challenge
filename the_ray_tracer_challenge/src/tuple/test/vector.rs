#[cfg(test)]
mod tests {
    use crate::tuple::model::Tuple;

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
}
