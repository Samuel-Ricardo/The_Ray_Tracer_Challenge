#[cfg(test)]
mod tests {
    use crate::point::model::Point;

    #[test]
    fn create_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
        assert_eq!(p.w, 1.0);
    }
}
