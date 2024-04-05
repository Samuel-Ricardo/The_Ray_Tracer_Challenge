mod point;
mod vector;

#[cfg(test)]
mod tests {
    use crate::tuple::model::Tuple;

    #[test]
    fn tuple_initialize_correctly() {
        let t = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(t.x, 1.0);
        assert_eq!(t.y, 2.0);
        assert_eq!(t.z, 3.0);
        assert_eq!(t.w, 4.0);
    }

    #[test]
    fn sum_of_two_tuples() {
        let t1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple::new(-2.0, 3.0, 1.0, 0.5);

        assert_eq!(t1 + t2, Tuple::new(1.0, 1.0, 6.0, 1.5));
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

    #[test]
    fn substracting_vector_from_point() {
        let t1 = Tuple::Point(3.0, -2.0, 5.0);
        let t2 = Tuple::Vector(-2.0, 3.0, 1.0);

        let expected = Tuple::Point(5.0, -5.0, 4.0);
        let result = t1 - t2;

        assert_eq!(result, expected);
        assert!(result.is_point());
    }
}
