use crate::float_eq::EPSILON;
use crate::intersections::{Intersection, Intersections};
use crate::material::Material;
use crate::matrices::{Matrix, Matrix4};
use crate::point::Point;
use crate::primitives::Primitive;
use crate::primitives::PrimitiveShape::TriangleShape;
use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triangle {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
    pub e_hit_a: Vector3,
    pub e_hit_b: Vector3,
    pub normal: Vector3,
    pub transformation: Matrix4,
    pub transformation_inverse: Matrix4,
    pub transformation_inverse_transpose: Matrix4,
    pub material: Material,
}

impl Triangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(pa: Point, pb: Point, pc: Point) -> Self {
        let e1: Vector3 = (pb - pa).into();
        let e2: Vector3 = (pc - pa).into();

        // if we will have two vectors of zeros -> normal gonna be Vector3(NAN, NAN, NAN)
        let normal = (e2.cross(&e1)).normalize();

        Self {
            point_a: pa,
            point_b: pb,
            point_c: pc,
            e_hit_a: e1,
            e_hit_b: e2,
            normal,
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            transformation_inverse_transpose: Matrix4::identity(),
            material: Default::default(),
        }
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let dir_cross_b_hit = ray.direction.cross(&self.e_hit_b);
        let determinant = self.e_hit_a.dot(&dir_cross_b_hit);

        if determinant.abs() < EPSILON {
            return Intersections::new();
        }

        return Intersections::new()
            .with(vec![Intersection::new(1.0, TriangleShape(self.clone()))]);
    }

    fn normal(&self, _world: &Point) -> Vector3 {
        self.normal
    }

    fn material(&self) -> Material {
        todo!()
    }

    fn transformation(&self) -> &Matrix4 {
        todo!()
    }

    fn transformation_invert(&self) -> &Matrix4 {
        todo!()
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            point_a: Point::default(),
            point_b: Point::default(),
            point_c: Point::default(),
            e_hit_a: Default::default(),
            e_hit_b: Default::default(),
            normal: Default::default(),
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            transformation_inverse_transpose: Matrix4::identity(),
            material: Default::default(),
        }
    }
}

#[cfg(test)]
mod triangle_tests {
    use crate::intersections::Intersections;
    use crate::point::Point;
    use crate::primitives::triangle::Triangle;
    use crate::primitives::Primitive;
    use crate::ray::Ray;
    use crate::vector::Vector3;

    #[test]
    fn triangle_creation() {
        let triangle = Triangle::default();
        let expected_triangle =
            Triangle::from(Point::default(), Point::default(), Point::default());

        assert_eq!(expected_triangle.point_a, triangle.point_a);
        assert_eq!(expected_triangle.point_b, triangle.point_b);
        assert_eq!(expected_triangle.point_c, triangle.point_c);
    }

    #[test]
    fn triangle_creation_addition() {
        let triangle = Triangle::from(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let expected_hit_a = Vector3::new(-1.0, -1.0, 0.0);
        let expected_hit_b = Vector3::new(1.0, -1.0, 0.0);
        let expected_normal = Vector3::new(0.0, 0.0, -1.0);

        assert_eq!(expected_hit_a, triangle.e_hit_a);
        assert_eq!(expected_hit_b, triangle.e_hit_b);
        assert_eq!(expected_normal, triangle.normal);
    }

    #[test]
    fn triangle_normal() {
        let triangle = Triangle::from(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        let expected_normal = triangle.normal;

        assert_eq!(expected_normal, triangle.normal(&Point::new(0.0, 0.5, 0.0)));
        assert_eq!(
            expected_normal,
            triangle.normal(&Point::new(-0.5, 0.75, 0.0))
        );
        assert_eq!(
            expected_normal,
            triangle.normal(&Point::new(0.5, 0.25, 0.0))
        );
    }

    #[test]
    fn triangle_intersect_ray_parallel() {
        let triangle = Triangle::from(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        let ray = Ray::new(Point::new(0.0, -1.0, -2.0), Vector3::new(0.0, 1.0, 0.0));
        let expected_intersections = Intersections::new();

        let intersections = triangle.intersect(&ray);

        assert_eq!(expected_intersections, intersections);
    }
}
