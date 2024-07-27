#[cfg(test)]
mod tests {
    use crate::canvas::model::color::Color;

    fn create_new_rgb_color() {
        let c = Color::new(2.0, 0.5, 1.0);

        assert_eq!(c.red, 2.0);
        assert_eq!(c.green, 0.5);
        assert_eq!(c.blue, 1.0);
    }

    fn clamp_color() {
        let color = Color::new(-0.5, 0.5, 1.5);

        let expected_result = Color::new(1.0, 0.0, 0.8);
        let actual_result = color.clamp(0.0, 1.0);

        assert_eq!(actual_result, expected_result);
    }
}
