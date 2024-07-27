#[cfg(test)]
mod ppm_tests {
    use crate::canvas::model::{ppm::PpmConvertible, Canvas};

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

        let EXPECTED_RESULT =
            format!("{FILE_TYPE_IDENTIFIER}\n{WIDTH} {HEIGHT}\n{MAX_COLOR_VALUE}\n");

        assert_eq!(EXPECTED_RESULT.as_bytes(), actual_result);
    }
}
