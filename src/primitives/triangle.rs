use crate::point::Point;
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

    #[test]
    fn triangle_creation() {
        let triangle = Triangle::default();
        let expected_triangle =
            Triangle::from(Point::default(), Point::default(), Point::default());

        assert_eq!(expected_triangle, triangle);
    }
}
