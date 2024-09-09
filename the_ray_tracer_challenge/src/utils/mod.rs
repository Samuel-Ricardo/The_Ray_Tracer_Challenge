mod equality;

pub fn f64_fuzzy_eq(left: f64, right: f64) -> bool {
    const EPSILON: f64 = 0.00001;
    (left - right).abs() < EPSILON
}
