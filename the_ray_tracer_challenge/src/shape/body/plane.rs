use std::f64::EPSILON;

use crate::{
    matrix::model::Matrix,
    ray_tracer::ray::Ray,
    shape::{intersect::intersectables::Intersectable, material::Material},
};

use super::Body;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Plane {
    material: Material,
    transform: Matrix<4>,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            material: Default::default(),
            transform: Matrix::identity(),
        }
    }
}

impl Plane {
    pub fn new(material: Material, transform: Matrix<4>) -> Self {
        Self {
            material,
            transform,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        return self;
    }

    pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        return self;
    }
}

impl Intersectable for Plane {
    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }

    fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(f64, Body)> {
        if object_space_ray.direction.y.abs() <= EPSILON {
            return vec![];
        }

        let result = -object_space_ray.origin.y / object_space_ray.direction.y;
        return vec![(result, Body::from(*self))];
    }
}
