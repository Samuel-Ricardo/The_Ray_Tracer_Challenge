#[cfg(test)]
mod ppm_tests {
    use crate::canvas::model::Canvas;

    #[test]
    fn build_ppm_header() {
        let canvas = Canvas::new(5, 3);
        // TODO: IMPLEMENT
        let ppm_image = canvas.to_ppm();
        let actual_result = &ppm_image[..11];

        const FILE_TYPE_IDENTIFIER: &str = "P3";
        const WIDTH: &str = "5";
        const HEIGHT: &str = "3";
        const MAX_COLOR_VALUE: &str = "255";

        const EXPECTED_RESULT: &str = "P3\n5 3\n255\n";

        assert_eq!(EXPECTED_RESULT.as_bytes(), actual_result);
    }
}
