use crate::tuple::model::Tuple;

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
}
