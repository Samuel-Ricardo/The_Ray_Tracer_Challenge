use crate::{matrix::model::Matrix, shape::material::Material};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Plane {
    material: Material,
    transform: Matrix<4>,
}

impl Plane {
    pub fn new(material: Material, transform: Matrix<4>) -> Self {
        Self {
            material,
            transform,
        }
    }
}
