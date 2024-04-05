#[cfg(test)]
mod tests {
    use crate::tuple::model::Tuple;

    #[test]
    fn point_initialize_correctly() {
        let p = Tuple::Point(1.0, 2.0, 3.0);

        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);

        assert_eq!(p.w, 1.0);
    }

    #[test]
    fn is_pointer_verifier() {
        let p = Tuple::Point(1.0, 2.0, 3.0);

        assert_eq!(p.is_point(), true);
    }

    #[test]
    fn not_is_pointer_verifier() {
        let p = Tuple::Vector(1.0, 2.0, 3.0);

        assert_eq!(p.is_point(), false);
    }

    #[test]
    fn sub_of_two_points() {
        let t1 = Tuple::Point(3.0, -2.0, 5.0);
        let t2 = Tuple::Point(-2.0, 3.0, 1.0);

        let expected = Tuple::Vector(5.0, -5.0, 4.0);
        let result = t1 - t2;

        assert_eq!(result, expected);
        assert!(result.is_vector());
    }
}
