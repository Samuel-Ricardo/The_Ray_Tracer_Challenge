use std::ops::Add;

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
