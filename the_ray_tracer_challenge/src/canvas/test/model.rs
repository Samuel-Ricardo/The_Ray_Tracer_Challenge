#[cfg(test)]
mod tests {
    use crate::canvas::model::{color::Color, Canvas};

    #[test]
    fn create_new_canvas() {
        let c = Canvas::new(64, 48);

        assert_eq!(64, c.width);
        assert_eq!(48, c.height);

        for x in 0..c.width - 1 {
            for y in 0..c.height - 1 {
                assert_eq!(*c.pixel_at(x, y), Color::black());
            }
        }
    }

    #[test]
    fn write_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 28);

        let red = Color::new(1.0, 0.0, 0.0);
        let expected = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_eq!(*c.pixel_at(2, 3), expected);
    }
}
