#[cfg(test)]
mod test {
    use crate::canvas::model::color::Color;

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(Color::new(1.6, 0.7, 1.0), c1 + c2);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(Color::new(0.2, 0.5, 0.5), c1 - c2);
    }
}
