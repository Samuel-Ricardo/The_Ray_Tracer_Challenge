#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn clamp(&self, lower_bound: f64, upper_bound: f64) -> Color {
        Color::new(
            self.red.min(upper_bound).max(lower_bound),
            self.green.min(upper_bound).max(lower_bound),
            self.blue.min(upper_bound).max(lower_bound),
        )
    }
}
