use crate::matrix::model::Matrix;

impl Matrix<2> {
    pub fn determinant(&self) -> f64 {
        (self[0][0] * self[1][1]) - (self[0][1] * self[1][0])
    }
}
