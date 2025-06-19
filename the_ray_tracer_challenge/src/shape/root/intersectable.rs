use crate::{matrix::model::Matrix, ray_tracer::ray::Ray, tuple::model::Tuple};

pub trait Intersectable {
    fn transform(&self) -> Matrix<4>;
    fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(f64, Body)>;
    fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple;
}
