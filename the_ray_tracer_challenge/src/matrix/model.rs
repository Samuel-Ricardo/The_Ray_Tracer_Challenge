use std::ops::{Index, IndexMut};

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

impl<const D: usize> Index<usize> for Matrix<D> {
    type Output = [f64; D];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const D: usize> IndexMut<usize> for Matrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const D: usize> FuzzyEq<Self> for Matrix<D> {
    fn fuzzy_eq(&self, other: Self) -> bool {
        for row in 0..D {
            for column in 0..D {
                if self[row][column].fuzzy_ne(other[row][column]) {
                    return false;
                }
            }
        }

        true
    }
}
