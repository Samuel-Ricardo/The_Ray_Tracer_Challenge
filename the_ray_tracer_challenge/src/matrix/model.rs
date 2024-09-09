use crate::utils::equality::FuzzyEq;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const D: usize> {
    data: [[f64; D]; D],
}

impl<const D: usize> From<[[f64; D]; D]> for Matrix<D> {
    fn from(data: [[f64; D]; D]) -> Self {
        Matrix { data }
    }
}

impl<const D: usize> Matrix<D> {
    pub fn new() -> Matrix<D> {
        Matrix::from([[0.0; D]; D])
    }
}

impl<const D: usize> FuzzyEq<Self> for Matrix<D> {
    fn fuzzy_eq(&self, other: Self) -> bool {
        for row in 0..D {
            for column in 0..D {
                if self.data[row][column].fuzzy_ne(other.data[row][column]) {
                    return false;
                }
            }
        }

        true
    }
}
