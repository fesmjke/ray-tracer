use crate::float_eq::LOW_EPSILON;
use crate::intersections::Intersection;
use crate::point::Point;
use crate::primitives::{Primitive, PrimitiveShape};
use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug, PartialEq, Clone)]
pub struct IntersectionDetails {
    pub time: f64,
    pub object: PrimitiveShape,
    pub point: Point,
    pub over_point: Point,
    pub normal_vector: Vector3,
    pub eye_vector: Vector3,
    pub reflection_vector: Vector3,
    pub inside: bool,
}

impl IntersectionDetails {
    pub fn from(intersection: &Intersection, ray: &Ray) -> Self {
        let point = ray.position(intersection.time);
        let eye_vector = -ray.direction;
        let mut normal_vector = intersection.object.normal(point);

        let inside = if normal_vector.dot(&eye_vector) < 0.0 {
            normal_vector = -normal_vector;
            true
        } else {
            false
        };

        let over_point = point + normal_vector * LOW_EPSILON;
        let reflection_vector = ray.direction.reflect(&normal_vector);

        Self {
            time: intersection.time,
            point,
            over_point,
            object: intersection.object.clone(),
            normal_vector,
            eye_vector,
            reflection_vector,
            inside,
        }
    }

    // fn is_inside(mut self) -> Self {
    //     if self.normal_vector.dot(&self.eye_vector) < 0.0 {
    //         self.normal_vector = -self.normal_vector;
    //         self.inside = true;
    //     } else {
    //         self.inside = false;
    //     }
    //
    //     self
    // }
}

#[cfg(test)]
mod intersection_details_tests {
    use crate::float_eq::LOW_EPSILON;
    use crate::intersections::{Intersection, IntersectionDetails};
    use crate::point::Point;
    use crate::primitives::PrimitiveShape::{PlaneShape, SphereShape};
    use crate::primitives::{Plane, Sphere};
    use crate::ray::Ray;
    use crate::transformations::Transformable;
    use crate::vector::Vector3;

    #[test]
    fn intersection_details_creation() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = SphereShape(Sphere::default());
        let intersection = Intersection::new(4.0, sphere.clone());
        let expected_details = IntersectionDetails {
            time: 4.0,
            point: Point::new(0.0, 0.0, -1.0),
            object: sphere.clone(),
            eye_vector: Vector3::new(0.0, 0.0, -1.0),
            normal_vector: Vector3::new(0.0, 0.0, -1.0),
            inside: false,
            over_point: Point::new(0.0, 0.0, -1.0001),
            reflection_vector: Vector3::new(0.0, 0.0, -1.0),
        };

        assert_eq!(
            expected_details,
            IntersectionDetails::from(&intersection, &ray)
        );
    }

    #[test]
    fn intersection_details_occurs_on_the_inside() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = SphereShape(Sphere::default());
        let intersection = Intersection::new(1.0, sphere.clone());
        let expected_eye_vector = Vector3::new(0.0, 0.0, -1.0);
        let expected_normal_vector = Vector3::new(0.0, 0.0, -1.0);

        let intersection_details = IntersectionDetails::from(&intersection, &ray);

        assert_eq!(expected_eye_vector, intersection_details.eye_vector);

        assert_eq!(expected_normal_vector, intersection_details.normal_vector);
    }

    #[test]
    fn intersection_details_should_offset_point() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = SphereShape(Sphere::default().translate(0.0, 0.0, 1.0).transform());
        let intersection = Intersection::new(5.0, sphere);
        let intersection_details = IntersectionDetails::from(&intersection, &ray);
        let expected_position_z = -LOW_EPSILON / 2.0;

        assert!(expected_position_z > intersection_details.over_point.z);
        assert!(intersection_details.point.z > intersection_details.over_point.z);
    }

    #[test]
    fn intersection_details_reflection_vector() {
        let ray = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let plane = PlaneShape(Plane::default());
        let intersection = Intersection::new(f64::sqrt(2.0), plane);
        let intersection_details = IntersectionDetails::from(&intersection, &ray);
        let expected_reflection_vector =
            Vector3::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0);

        assert_eq!(
            expected_reflection_vector,
            intersection_details.reflection_vector
        );
    }
}
