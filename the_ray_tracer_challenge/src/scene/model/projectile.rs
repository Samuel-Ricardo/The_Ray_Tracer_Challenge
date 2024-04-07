use crate::tuple::model::Tuple;

#[derive(Debug)]
pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }
}
