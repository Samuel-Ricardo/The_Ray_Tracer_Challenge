#[cfg(test)]
mod test {
    use crate::vector::model::Vector;

    #[test]
    fn vector_initiate_correctly() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
        assert_eq!(vector.w, 0.0);
    }
}
