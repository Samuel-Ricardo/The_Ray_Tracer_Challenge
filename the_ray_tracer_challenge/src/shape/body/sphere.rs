use crate::{matrix::model::Matrix, shape::material::Material, utils::equality::FuzzyEq};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Default::default(),
        }
    }
}

impl Sphere {
    pub fn new(material: Material, transform: Matrix<4>) -> Self {
        Sphere {
            transform,
            material,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        return self;
    }

    pub fn with_transform(mut self, transfor: Matrix<4>) -> Self {
        self.transform = transfor;
        return self;
    }
}

impl FuzzyEq<&Sphere> for Sphere {
    fn fuzzy_eq(&self, other: &Sphere) -> bool {
        self.transform.fuzzy_eq(other.transform) && self.material.fuzzy_eq(other.material)
    }
}
