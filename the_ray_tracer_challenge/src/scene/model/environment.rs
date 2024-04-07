use crate::tuple::model::Tuple;

#[derive(Debug)]
pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self { gravity, wind }
    }
}
