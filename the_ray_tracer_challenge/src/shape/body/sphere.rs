use crate::{
    matrix::model::Matrix,
    ray_tracer::ray::Ray,
    shape::{intersect::intersectables::Intersectable, material::Material},
    tuple::model::Tuple,
    utils::equality::FuzzyEq,
};

use super::Body;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Default::default(),
        }
    }
}

impl Sphere {
    pub fn new(material: Material, transform: Matrix<4>) -> Self {
        Sphere {
            transform,
            material,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        return self;
    }

    pub fn with_transform(mut self, transfor: Matrix<4>) -> Self {
        self.transform = transfor;
        return self;
    }
}

impl FuzzyEq<&Sphere> for Sphere {
    fn fuzzy_eq(&self, other: &Sphere) -> bool {
        self.transform.fuzzy_eq(other.transform) && self.material.fuzzy_eq(other.material)
    }
}

impl Intersectable for Sphere {
    fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(crate::f64, Body)> {
        let sphere_to_ray = object_space_ray.origin - Tuple::zero_point();

        let a = object_space_ray.direction.dot(&object_space_ray.direction);
        let b = 2.0 * object_space_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let descriminant = b.powi(2) - 4.0 * a * c;

        if descriminant < 0.0 {
            vec![]
        } else {
            let first_intersection = (-b - descriminant.sqrt()) / (2.0 * a);
            let second_intersection = (-b + descriminant.sqrt()) / (2.0 * a);

            vec![
                (first_intersection, Body::from(*self)),
                (second_intersection, Body::from(*self)),
            ]
        }
    }

    fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple {
        (object_space_point - Tuple::zero_point()).normalize()
    }

    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Matrix<4> {
        self.transform
    }
}
