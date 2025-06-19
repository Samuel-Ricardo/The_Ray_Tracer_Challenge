use std::f64::EPSILON;

use crate::{ray_tracer::ray::Ray, shape::root::body::Body};

use super::computed::ComputedIntersection;

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

    pub fn get_computed(&self) -> ComputedIntersection {
        let position = self.ray.position(self.distance);
        let mut normal_v = self.body.normal_at(position);
        let eye_v = -self.ray.direction;
        let inside = normal_v.dot(eye_v) < 0.0;

        if inside {
            normal_v = -normal_v;
        }

        let over_point = position + normal_v * EPSILON;
        let reflect_v = self.ray.direction.reflect(normal_v);

        return ComputedIntersection::new(
            self, position, over_point, normal_v, eye_v, reflect_v, inside,
        );
    }
}

pub struct Intersections {
    pub data: Vec<Intersection>,
}

impl Intersections {
    pub fn new(mut intersections: Vec<Intersection>) -> Self {
        intersections.sort_unstable_by(|first, second| {
            first.distance.partial_cmp(&second.distance).unwrap()
        });

        return Intersections {
            data: intersections,
        };
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}
