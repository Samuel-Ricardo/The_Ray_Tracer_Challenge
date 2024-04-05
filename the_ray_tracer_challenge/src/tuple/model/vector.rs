use super::Tuple;

impl Tuple {
    pub fn Vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}
