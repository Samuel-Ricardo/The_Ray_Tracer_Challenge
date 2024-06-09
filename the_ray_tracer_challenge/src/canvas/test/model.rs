#[cfg(test)]
mod canvas_model_tests {
    use crate::canvas::model::{color::Color, Canvas};

    #[test]
    fn create_new_canvas() {
        let c = Canvas::new(64, 48);

        assert_eq!(64, c.width);
        assert_eq!(48, c.height);

        for x in 0..c.width - 1 {
            for y in 0..c.height - 1 {
                assert_eq!(*c.color_at(x, y), Color::black());
            }
        }
    }
}
