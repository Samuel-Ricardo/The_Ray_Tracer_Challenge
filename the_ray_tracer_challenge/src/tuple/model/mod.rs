mod math;
mod point;
mod vector;

use crate::utils::equality::FuzzyEq;

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }
}

impl FuzzyEq<&Tuple> for Tuple {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }

    fn fuzzy_ne(&self, other: &Self) -> bool {
        !self.fuzzy_eq(other)
    }
}
