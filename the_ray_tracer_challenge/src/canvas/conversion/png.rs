use super::rgba32;
use crate::canvas::model::Sized;

pub trait Convertable {
    fn to_png(&self) -> Vec<u8>;
}

impl<T> Convertable for T
where
    T: rgba32::Convertible,
    T: Sized,
{
    fn to_png(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut encoder = png::Encoder::new(&mut data, self.width() as u32, self.height() as u32);

        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writter = encoder.write_header().unwrap();
        writter.write_image_data(&self.to_rgba32()).unwrap();
        drop(writter);

        return data;
    }
}
