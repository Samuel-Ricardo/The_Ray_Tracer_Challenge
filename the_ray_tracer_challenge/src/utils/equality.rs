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

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, $right) {
            (left_val, right_val) => {
                if left_val.fuzzy_ne(right_val.clone()) {
                    panic!(
                        "assertion failed they are not fuzzy equal: `{:?}` != `{:?}`",
                        left_val, right_val
                    );
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_fuzzy_ne {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, $right) {
            (left_val, right_val) => {
                if left_val.fuzzy_eq(right_val.clone()) {
                    panic!(
                        "assertion failed they are fuzzy equal: `{:?}` == `{:?}`",
                        left_val, right_val
                    );
                }
            }
        }
    };
}
