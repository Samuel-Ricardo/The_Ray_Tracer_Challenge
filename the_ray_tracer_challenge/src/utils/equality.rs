pub trait FuzzyEq<T: Clone> {
    fn fuzzy_eq(&self, right: T) -> bool;
    fn fuzzy_ne(&self, right: T) -> bool {
        !self.fuzzy_eq(right)
    }
}

pub fn f64_fuzzy_eq(left: f64, right: f64) -> bool {
    const EPSILON: f64 = 0.00001;
    (left - right).abs() < EPSILON
}
