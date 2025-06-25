use crate::{matrix::model::Matrix, shape::material::Material};

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
}
