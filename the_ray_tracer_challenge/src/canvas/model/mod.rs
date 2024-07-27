use self::color::Color;
use self::ppm::PpmConvertible;

pub mod color;
pub mod ppm;

pub trait Sized {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,

    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let index = y * self.width + x;
        self.pixels[index]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.get_pixel_index(x, y);
        self.pixels[index] = color;
    }

    fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn build_ppm_pixel_data(&self) -> Vec<u8> {
        let mut pixel_strings: Vec<String> = Vec::new();

        for pixel in self.pixels.iter() {
            let clamped_color = pixel.clamp(0.0, 1.0);

            let red: u8 = (clamped_color.red * 255.0).round() as u8;
            let green: u8 = (clamped_color.green * 255.0).round() as u8;
            let blue: u8 = (clamped_color.blue * 255.0).round() as u8;

            //            pixel_string.push(format!("{red} {green} {blue} ").as_bytes());
            pixel_strings.push(format!("{red}"));
            pixel_strings.push(format!("{green}"));
            pixel_strings.push(format!("{blue}"));
        }

        let mut pixel_data: Vec<u8> = Vec::new();
        let mut column_count: usize = 0;

        for pixel_string in pixel_strings.iter() {
            let mut needed_space: usize = 0;

            if column_count != 0 {
                needed_space += 1;
            }

            needed_space += pixel_string.len();

            if column_count + needed_space > 70 {
                pixel_data.extend(String::from("\n").into_bytes());
                column_count = 0;
            }

            if column_count != 0 {
                pixel_data.extend(String::from(" ").into_bytes());
            }

            pixel_data.extend(pixel_string.clone().into_bytes());
        }

        pixel_data.extend(String::from("\n").into_bytes());

        return pixel_data;
    }
}

impl Sized for Canvas {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl PpmConvertible for Canvas {
    fn to_ppm(&self) -> Vec<u8> {
        let header = self.build_ppm_header();
        let pixel_data = self.build_ppm_pixel_data();

        let mut ppm = Vec::new();
        ppm.extend(header);
        ppm.extend(pixel_data);

        return ppm;
    }
}
