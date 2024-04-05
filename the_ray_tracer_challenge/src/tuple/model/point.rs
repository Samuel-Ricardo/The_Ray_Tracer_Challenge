use super::Tuple;

impl Tuple {
    pub fn Point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
}
