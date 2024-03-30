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
}
