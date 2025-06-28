use crate::{matrix::model::Matrix, shape::material::Material};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Plane {
    material: Material,
    transform: Matrix<4>,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            material: Default::default(),
            transform: Matrix::identity(),
        }
    }
}

impl Plane {
    pub fn new(material: Material, transform: Matrix<4>) -> Self {
        Self {
            material,
            transform,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        return self;
    }
}
