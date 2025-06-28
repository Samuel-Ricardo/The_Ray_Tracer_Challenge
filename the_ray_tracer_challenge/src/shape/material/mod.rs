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

impl From<Phong> for Material {
    fn from(phong: Phong) -> Self {
        Material::Phong(phong)
    }
}

impl FuzzyEq<Material> for Material {
    fn fuzzy_eq(&self, other: Material) -> bool {
        match (self, other) {
            (Material::Phong(ref m), Material::Phong(other)) => m.fuzzy_eq(other),
        }
    }
}
