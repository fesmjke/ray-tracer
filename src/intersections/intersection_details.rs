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
    pub normal_vector: Vector3,
    pub eye_vector: Vector3,
}

impl IntersectionDetails {
    pub fn from(intersection: Intersection, ray: &Ray) -> Self {
        let point = ray.position(intersection.time);
        let eye_vector = -ray.direction;
        let normal_vector = intersection.object.normal(point);

        Self {
            time: intersection.time,
            point,
            object: intersection.object,
            normal_vector,
            eye_vector,
        }
    }
}

#[cfg(test)]
mod intersection_details_tests {
    use crate::intersections::{Intersection, IntersectionDetails};
    use crate::point::Point;
    use crate::primitives::PrimitiveShape::SphereShape;
    use crate::primitives::Sphere;
    use crate::ray::Ray;
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
        };

        assert_eq!(
            expected_details,
            IntersectionDetails::from(intersection, &ray)
        );
    }
}
