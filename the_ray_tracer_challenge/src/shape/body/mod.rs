pub mod plane;
pub mod sphere;

use self::{plane::Plane, sphere::Sphere};

use super::intersect::intersectables::Intersectable;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Body {
    Sphere(Sphere),
    Plane(Plane),
}

impl From<Sphere> for Body {
    fn from(sphere: Sphere) -> Self {
        Body::Sphere(sphere)
    }
}

impl From<Plane> for Body {
    fn from(plane: Plane) -> Self {
        Body::Plane(plane)
    }
}

impl Intersectable for Body {
    fn normal_at(&self, point: crate::tuple::model::Tuple) -> crate::tuple::model::Tuple {
        todo!()
    }

    fn material(&self) -> super::material::Material {
        todo!()
    }

    fn transform(&self) -> crate::matrix::model::Matrix<4> {
        todo!()
    }

    fn intersect_in_object_space(
        &self,
        object_space_ray: crate::ray_tracer::ray::Ray,
    ) -> Vec<(f64, Self)> {
        todo!()
    }

    fn normal_at_in_object_space(
        &self,
        object_space_point: crate::tuple::model::Tuple,
    ) -> crate::tuple::model::Tuple {
        todo!()
    }
}
