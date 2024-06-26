use crate::utils::f64_fuzzy_eq;

use super::Tuple;
use std::ops;

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        f64_fuzzy_eq(self.x, other.x)
            && f64_fuzzy_eq(self.y, other.y)
            && f64_fuzzy_eq(self.z, other.z)
            && f64_fuzzy_eq(self.w, other.w)
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Self::Output {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f64) -> Self::Output {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, other: f64) -> Self::Output {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}
