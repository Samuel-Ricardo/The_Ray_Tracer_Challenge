use self::phong::Phong;

pub mod pattern;
pub mod phong;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Material {
    Phong(Phong),
}
