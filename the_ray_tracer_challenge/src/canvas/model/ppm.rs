use super::Sized;

const FILE_TYPE_IDENTIFIER: &str = "P3\n";
const MAX_COLOR_VALUE: &str = "255\n";

pub trait PpmConvertible {
    fn build_ppm_header(&self) -> Vec<u8>
    where
        Self: Sized,
    {
        let mut header = Vec::new();

        header.extend(FILE_TYPE_IDENTIFIER.as_bytes());
        header.extend(format!("{} {}\n", self.width(), self.height()).into_bytes());
        header.extend(MAX_COLOR_VALUE.as_bytes());

        header
    }

    fn to_ppm(&self) -> Vec<u8>;
}
