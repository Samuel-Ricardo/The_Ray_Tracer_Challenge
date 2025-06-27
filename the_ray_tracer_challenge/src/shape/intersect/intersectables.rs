use crate::{
    matrix::model::Matrix,
    ray_tracer::ray::Ray,
    shape::{body::Body, material::Material},
    tuple::model::Tuple,
};

use super::intersection::{Intersection, Intersections};

pub trait Intersectable {
    fn material(&self) -> Material;
    fn transform(&self) -> Matrix<4>;
    fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(f64, Body)>;
    fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple;

    fn intersect(&self, ray: Ray) -> Intersections {
        let object_space_ray = ray.transform(self.transform().inverse());
        let ts = self.intersect_in_object_space(object_space_ray);

        Intersections::new(
            ts.into_iter()
                .map(|(t, body)| Intersection::new(t, ray, body))
                .collect(),
        )
    }

    fn normal_at(&self, point: Tuple) -> Tuple {
        let object_space_point = self.transform().inverse() * point;
        let object_normal = self.normal_at_in_object_space(object_space_point);

        let mut world_normal = self.transform().inverse().transpose() * object_normal;

        world_normal.w = 0.0;
        world_normal.normalize()
    }
}
