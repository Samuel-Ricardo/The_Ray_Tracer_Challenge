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
}
