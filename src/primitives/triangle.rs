use crate::intersections::Intersections;
use crate::material::Material;
use crate::matrices::Matrix4;
use crate::point::Point;
use crate::primitives::Primitive;
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
        }
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &Ray) -> Intersections {
        todo!()
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
        }
    }
}

#[cfg(test)]
mod triangle_tests {
    use crate::point::Point;
    use crate::primitives::triangle::Triangle;
    use crate::primitives::Primitive;
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
}
