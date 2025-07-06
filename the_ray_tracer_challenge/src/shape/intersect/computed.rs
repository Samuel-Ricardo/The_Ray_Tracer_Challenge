use super::intersection::Intersection;
use crate::tuple::model::Tuple;

#[derive(Debug, Clone)]
pub struct ComputedIntersection<'i> {
    pub intersection: &'i Intersection,
    pub point: Tuple,
    pub over_point: Tuple,
    pub normal_v: Tuple,
    pub eye_v: Tuple,
    pub reflect_v: Tuple,
    pub inside: bool,
}

impl<'i> ComputedIntersection<'i> {
    pub fn new(
        intersection: &'i Intersection,
        point: Tuple,
        over_point: Tuple,
        normal_v: Tuple,
        eye_v: Tuple,
        reflect_v: Tuple,
        inside: bool,
    ) -> Self {
        ComputedIntersection {
            intersection,
            point,
            over_point,
            normal_v,
            eye_v,
            reflect_v,
            inside,
        }
    }
}
