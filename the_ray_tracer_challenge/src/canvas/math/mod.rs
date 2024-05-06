use std::ops::{Add, Mul, Sub};

use crate::utils::f64_fuzzy_eq;

use super::model::color::Color;

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Self) -> Self::Output {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self::Output {
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        f64_fuzzy_eq(self.red, other.red)
            && f64_fuzzy_eq(self.green, other.green)
            && f64_fuzzy_eq(self.blue, other.blue)
    }
}
