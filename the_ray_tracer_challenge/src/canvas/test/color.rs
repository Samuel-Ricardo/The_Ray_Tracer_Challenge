#[cfg(test)]
mod tests {
    use crate::canvas::model::color::Color;

    fn create_new_rgb_color() {
        let c = Color::new(2.0, 0.5, 1.0);

        assert_eq!(c.red, 2.0);
        assert_eq!(c.green, 0.5);
        assert_eq!(c.blue, 1.0);
    }
}
