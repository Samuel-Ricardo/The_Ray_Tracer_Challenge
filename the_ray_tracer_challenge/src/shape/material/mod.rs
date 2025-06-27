use crate::utils::equality::FuzzyEq;

use self::phong::Phong;

pub mod pattern;
pub mod phong;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Material {
    Phong(Phong),
}

impl Default for Material {
    fn default() -> Self {
        Material::from(Phong::default())
    }
}
