pub trait FuzzyEq<T: Clone> {
    fn fuzzy_eq(&self, right: T) -> bool;
    fn fuzzy_ne(&self, right: T) -> bool {
        !self.fuzzy_eq(right)
    }
}
