use crate::point::Point;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Triangle {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
}

impl Triangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(pa: Point, pb: Point, pc: Point) -> Self {
        Self {
            point_a: pa,
            point_b: pb,
            point_c: pc,
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            point_a: Point::default(),
            point_b: Point::default(),
            point_c: Point::default(),
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
