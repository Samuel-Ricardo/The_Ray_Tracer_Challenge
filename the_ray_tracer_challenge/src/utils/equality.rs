use super::EPSILON;

pub trait FuzzyEq<T: Clone> {
    fn fuzzy_eq(&self, other: T) -> bool;
    fn fuzzy_ne(&self, other: T) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl FuzzyEq<f64> for f64 {
    fn fuzzy_eq(&self, other: f64) -> bool {
        (*self - other).abs() < EPSILON
    }
}
