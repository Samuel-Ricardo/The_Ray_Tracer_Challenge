#[cfg(test)]
mod ray_test {
    use crate::{matrix::model::Matrix, raytracer::ray::Ray, tuple::model::Tuple};

    #[test]
    fn create_and_query_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let origin = Tuple::point(2.0, 3.0, 4.0);
        let direction = Tuple::vector(1.0, 0.0, 0.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(0.0, 1.0, 0.0);

        let ray = Ray::new(origin, direction);
        let translate = Matrix::translation(3.0, 4.0, 5.0);

        let translated_ray = ray.transform(translate);

        assert_eq!(Tuple::point(4.0, 6.0, 8.0), translated_ray.origin);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0), translated_ray.direction);
    }
}
