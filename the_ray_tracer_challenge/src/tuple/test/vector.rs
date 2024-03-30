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
}
