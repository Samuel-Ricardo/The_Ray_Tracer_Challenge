use crate::utils::equality::FuzzyEq;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Patter {}

impl FuzzyEq<Patter> for Patter {
    fn fuzzy_eq(&self, _other: Patter) -> bool {
        true
    }
}
