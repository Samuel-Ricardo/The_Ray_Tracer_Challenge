#[cfg(test)]
mod tests {
    use crate::canvas::model::{color::Color, ppm::PpmConvertible, Canvas};

    const FILE_TYPE_IDENTIFIER: &str = "P3";
    const WIDTH: &str = "5";
    const HEIGHT: &str = "3";
    const MAX_COLOR_VALUE: &str = "255";

    #[test]
    fn build_ppm_header() {
        let canvas = Canvas::new(5, 3);
        // TODO: IMPLEMENT
        let ppm_image = canvas.to_ppm();
        let actual_result = &ppm_image[..11];

        let EXPECTED_RESULT =
            format!("{FILE_TYPE_IDENTIFIER}\n{WIDTH} {HEIGHT}\n{MAX_COLOR_VALUE}\n");

        assert_eq!(EXPECTED_RESULT.as_bytes(), actual_result);
    }

    #[test]
    fn constructing_the_pixel_data() {
        let mut canvas = Canvas::new(5, 3);

        let color1 = Color::new(1.5, 0.0, 0.0);
        let color2 = Color::new(0.0, 0.5, 0.0);
        let color3 = Color::new(-0.5, 0.0, 1.0);

        canvas.write_pixel(0, 0, color1);
        canvas.write_pixel(2, 1, color2);
        canvas.write_pixel(4, 2, color3);

        let actual_result = canvas.to_ppm();

        let header =
            format!("{FILE_TYPE_IDENTIFIER}\n{WIDTH} {HEIGHT}\n{MAX_COLOR_VALUE}\n").into_bytes();

        let pixel_data = String::from(
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n",
        ).into_bytes();

        let mut expected_result: Vec<u8> = Vec::new();
        expected_result.extend(header);
        expected_result.extend(pixel_data);

        assert_eq!(expected_result, actual_result);
    }
}
