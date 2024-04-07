use crate::tuple::model::Tuple;

#[derive(Debug)]
pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}
