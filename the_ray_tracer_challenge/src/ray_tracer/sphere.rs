use crate::{matrix::model::Matrix, tuple::model::Tuple};

use super::{intersection::Intersection, ray::Ray};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    pub transform: Matrix<4>,
}

impl Sphere {
    pub fn new(transform: Option<Matrix<4>>) -> Self {
        match transform {
            None => Sphere {
                transform: Matrix::identity(),
            },
            Some(transform) => Sphere { transform },
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let object_space_ray = ray.transform(self.transform.inverse());

        let sphere_to_ray = object_space_ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = object_space_ray.direction.dot(&object_space_ray.direction);
        let b = 2.0 * object_space_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let descriminant = b.powi(2) - 4.0 * a * c;

        if descriminant < 0.0 {
            return vec![];
        }

        let intersection1 = (-b - descriminant.sqrt()) / (2.0 * a);
        let intersection2 = (-b + descriminant.sqrt()) / (2.0 * a);

        vec![
            Intersection::new(intersection1),
            Intersection::new(intersection2),
        ]
    }
}
