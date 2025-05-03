use crate::matrix::model::Matrix;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix<4>,
}

impl Sphere {
    pub fn new(transform: Option<Matrix<4>>) -> Self {
        match transform {
            None => Sphere {
                transform: Matrix::identity(),
            },
            Some(transform) => Sphere { transform },
        }
    }
}
