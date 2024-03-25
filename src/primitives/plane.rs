use crate::float_eq::EPSILON;
use crate::intersections::{Intersection, Intersections};
use crate::material::Material;
use crate::matrices::{Matrix, Matrix4};
use crate::point::Point;
use crate::primitives::Primitive;
use crate::primitives::PrimitiveShape::PlaneShape;
use crate::ray::Ray;
use crate::transformations::Transformable;
use crate::vector::Vector3;
use std::default::Default;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    transformation: Matrix4,
    pub material: Material,
}

impl Plane {
    pub fn new(material: Material) -> Self {
        Self {
            transformation: Matrix4::identity(),
            material,
        }
    }

    pub fn apply_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}

impl Primitive for Plane {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let intersections = Intersections::new();

        if ray.direction.y.abs() <= EPSILON {
            intersections
        } else {
            intersections.with(vec![Intersection::new(
                -ray.origin.y / ray.direction.y,
                PlaneShape(self),
            )])
        }
    }

    fn normal(&self, world: &Point) -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0)
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn transformation(&self) -> &Matrix4 {
        &self.transformation
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            material: Material::default(),
            transformation: Matrix4::identity(),
        }
    }
}

impl Transformable for Plane {
    fn transform(self, transformation: &Matrix4) -> Plane {
        Self {
            transformation: *transformation * self.transformation,
            ..self
        }
    }
}

#[cfg(test)]
mod plane_tests {
    use crate::intersections::{Intersection, Intersections};
    use crate::material::Material;
    use crate::matrices::{Matrix, Matrix4};
    use crate::point::Point;
    use crate::primitives::PrimitiveShape::PlaneShape;
    use crate::primitives::{Plane, Primitive};
    use crate::ray::Ray;
    use crate::vector::Vector3;

    #[test]
    fn plane_creation() {
        let plane = Plane::default();
        let expected_plane = Plane {
            material: Material::default(),
            transformation: Matrix4::identity(),
        };

        assert_eq!(expected_plane, plane);
    }

    #[test]
    fn plane_intersect_with_parallel_ray() {
        let plane = Plane::default();
        let ray = Ray::new(Point::new(0.0, 10.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let intersections = plane.intersect(&ray);
        let expected_intersections = Intersections::new();

        assert_eq!(expected_intersections, intersections);
    }

    #[test]
    fn plane_intersect_with_coplanar_ray() {
        let plane = Plane::default();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let intersections = plane.intersect(&ray);
        let expected_intersections = Intersections::new();

        assert_eq!(expected_intersections, intersections);
    }

    #[test]
    fn plane_intersect_from_above() {
        let plane = Plane::default();
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0));

        let intersections = plane.intersect(&ray);
        let expected_intersections =
            Intersections::new().with(vec![Intersection::new(1.0, PlaneShape(&plane))]);

        assert_eq!(expected_intersections, intersections);
    }

    #[test]
    fn plane_intersect_from_below() {
        let plane = Plane::default();
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));

        let intersections = plane.intersect(&ray);
        let expected_intersections =
            Intersections::new().with(vec![Intersection::new(1.0, PlaneShape(&plane))]);

        assert_eq!(expected_intersections, intersections);
    }
}
