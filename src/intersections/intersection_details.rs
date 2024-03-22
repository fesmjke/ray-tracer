use crate::constants::REFRACTION_VACUUM;
use crate::float_eq::LOW_EPSILON;
use crate::intersections::{Intersection, Intersections};
use crate::point::Point;
use crate::primitives::{Primitive, PrimitiveShape};
use crate::ray::Ray;
use crate::vector::Vector3;
use std::collections::VecDeque;

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
    pub n1: f64,
    pub n2: f64,
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
            n1: 1.0, // TEMP
            n2: 1.0, // TEMP
        }
    }

    pub fn from_many(
        hit_intersection: &Intersection,
        intersections: &Intersections,
        ray: &Ray,
    ) -> Self {
        let point = ray.position(hit_intersection.time);
        let eye_vector = -ray.direction;
        let mut normal_vector = hit_intersection.object.normal(point);

        let inside = if normal_vector.dot(&eye_vector) < 0.0 {
            normal_vector = -normal_vector;
            true
        } else {
            false
        };

        let over_point = point + normal_vector * LOW_EPSILON;
        let reflection_vector = ray.direction.reflect(&normal_vector);

        let mut container: VecDeque<PrimitiveShape> = VecDeque::new();

        let mut n1 = f64::NAN;
        let mut n2 = f64::NAN;

        let default_refraction_index = REFRACTION_VACUUM;
        for intersection in intersections.intersections.iter() {
            if intersection == hit_intersection {
                n1 = match container.back() {
                    Some(o) => o.material().refractive_index,
                    None => default_refraction_index,
                };
            }

            if container.contains(&intersection.object) {
                let index = container
                    .iter()
                    .position(|x| *x == intersection.object)
                    .expect("INTERSECTION_DETAILS: unable to find object in vector!");

                container.remove(index);
            } else {
                container.push_back(intersection.object.clone())
            }

            if intersection == hit_intersection {
                n2 = match container.back() {
                    Some(o) => o.material().refractive_index,
                    None => default_refraction_index,
                };
                break;
            }
        }

        Self {
            time: hit_intersection.time,
            point,
            over_point,
            object: hit_intersection.object.clone(),
            normal_vector,
            eye_vector,
            reflection_vector,
            inside,
            n1,
            n2,
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
    use crate::intersections::{Intersection, IntersectionDetails, Intersections};
    use crate::material::Material;
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
            n1: 1.0,
            over_point: Point::new(0.0, 0.0, -1.0001),
            reflection_vector: Vector3::new(0.0, 0.0, -1.0),
            n2: 1.0,
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

    #[test]
    fn intersection_details_find_n1_n2() {
        let sphere_a = SphereShape(
            Sphere::default()
                .scale(2.0, 2.0, 2.0)
                .transform()
                .apply_material(Material::default().refractive_index(1.5).transparency(1.0)),
        );

        let sphere_b = SphereShape(
            Sphere::default()
                .translate(0.0, 0.0, -0.25)
                .transform()
                .apply_material(Material::default().refractive_index(2.0).transparency(1.0)),
        );

        let sphere_c = SphereShape(
            Sphere::default()
                .translate(0.0, 0.0, 0.25)
                .transform()
                .apply_material(Material::default().refractive_index(2.5).transparency(1.0)),
        );

        let ray = Ray::new(Point::new(0.0, 0.0, -4.0), Vector3::new(0.0, 0.0, 1.0));

        let intersection_a_1 = Intersection::new(2.0, sphere_a.clone());
        let intersection_b_1 = Intersection::new(2.75, sphere_b.clone());
        let intersection_c_1 = Intersection::new(3.25, sphere_c.clone());
        let intersection_b_2 = Intersection::new(4.75, sphere_b.clone());
        let intersection_c_2 = Intersection::new(5.25, sphere_c.clone());
        let intersection_a_2 = Intersection::new(6.0, sphere_a.clone());

        let intersections = Intersections::new().with(vec![
            intersection_a_1.clone(),
            intersection_b_1,
            intersection_c_1,
            intersection_b_2.clone(),
            intersection_c_2,
            intersection_a_2.clone(),
        ]);

        let intersection_details =
            IntersectionDetails::from_many(&intersection_a_1, &intersections, &ray);
        let expected_n1_n2 = (1.0, 1.5);

        assert_eq!(
            expected_n1_n2,
            (intersection_details.n1, intersection_details.n2)
        );

        let expected_n1_n2 = (2.5, 2.5);
        let intersection_details =
            IntersectionDetails::from_many(&intersection_b_2, &intersections, &ray);
        assert_eq!(
            expected_n1_n2,
            (intersection_details.n1, intersection_details.n2)
        );

        let expected_n1_n2 = (1.5, 1.0);
        let intersection_details =
            IntersectionDetails::from_many(&intersection_a_2, &intersections, &ray);
        assert_eq!(
            expected_n1_n2,
            (intersection_details.n1, intersection_details.n2)
        );
    }
}
