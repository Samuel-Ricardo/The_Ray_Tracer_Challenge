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
    fn vector_initialize_correctly() {
        let v = Tuple::Vector(1.0, 2.0, 3.0);

        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);

        assert_eq!(v.w, 0.0);
    }
}
