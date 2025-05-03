use crate::{matrix::model::Matrix, tuple::model::Tuple};

pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        if !origin.is_point() || !direction.is_vector() {
            panic!("Origin and direction must both be points and vectors");
        }

        Ray { origin, direction }
    }

    pub fn position(&self, distance: f64) -> Tuple {
        self.origin + self.direction * distance
    }

    pub fn transform(&self, m: Matrix<4>) -> Self {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}
