#[cfg(test)]
mod sphere_test {
    use crate::{
        matrix::model::Matrix,
        ray_tracer::ray::Ray,
        shape::{
            body::sphere::Sphere, intersect::intersectables::Intersectable, material::Material,
        },
        tuple::model::Tuple,
    };

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        let intersections = sphere.intersect(ray);

        assert_eq!(2, intersections.length());
        assert_eq!(4.0, intersections[0].distance);
        assert_eq!(6.0, intersections[1].distance);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        let intersections = sphere.intersect(ray);

        assert_eq!(2, intersections.length());
        assert_eq!(5.0, intersections[0].distance);
        assert_eq!(5.0, intersections[1].distance);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        let intersections = sphere.intersect(ray);

        assert_eq!(0, intersections.length());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        let intersections = sphere.intersect(ray);

        assert_eq!(2, intersections.length());
        assert_eq!(-1.0, intersections[0].distance);
        assert_eq!(1.0, intersections[1].distance);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::default();

        let intersections = sphere.intersect(ray);

        assert_eq!(2, intersections.length());
        assert_eq!(-6.0, intersections[0].distance);
        assert_eq!(-4.0, intersections[1].distance);
    }

    #[test]
    fn a_sphere_default_transform() {
        let sphere = Sphere::default();
        assert_eq!(Matrix::identity(), sphere.transform);
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut sphere = Sphere::default();
        let matrix = Matrix::translation(2.0, 3.0, 4.0);

        sphere.transform = matrix;

        assert_eq!(matrix, sphere.transform);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let origin = Tuple::point(0.0, 0.0, -5.0);
        let direction = Tuple::vector(0.0, 0.0, 1.0);

        let ray = Ray::new(origin, direction);
        let sphere = Sphere::default().with_transform(Matrix::scaling(2.0, 2.0, 2.0));

        let intersections = sphere.intersect(ray);

        assert_eq!(2, intersections.length());
        assert_eq!(3.0, intersections[0].distance);
        assert_eq!(7.0, intersections[1].distance);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let origin = Tuple::point(0.0, 0.0, -5.0);
        let direction = Tuple::vector(0.0, 0.0, 1.0);

        let ray = Ray::new(origin, direction);
        let sphere = Sphere::default().with_transform(Matrix::translation(5.0, 0.0, 0.0));

        let intersections = sphere.intersect(ray);

        assert_eq!(0, intersections.length());
    }
}
