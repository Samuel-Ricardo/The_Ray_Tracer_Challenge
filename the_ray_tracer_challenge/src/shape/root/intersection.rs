use crate::ray_tracer::ray::Ray;

use super::body::Body;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection {
    pub distance: f64,
    pub ray: Ray,
    pub body: Body,
}

impl Intersection {
    pub fn new(distance: f64, ray: Ray, body: Body) -> Self {
        Intersection {
            distance,
            ray,
            body,
        }
    }
}
