use crate::point::Point;
use crate::vector::Vector3;

// TODO: reference and lifetimes
struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    fn new(origin: Point, direction: Vector3) -> Self {
        Self { origin, direction }
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Point::default(),
            direction: Vector3::default(),
        }
    }
}

#[cfg(test)]
mod ray_tests {
    use super::*;

    #[test]
    fn ray_creation() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(1.0, 2.0, 3.0));
        let expected_origin = Point::new(0.0, 0.0, 0.0);
        let expected_direction = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(expected_origin, ray.origin);
        assert_eq!(expected_direction, ray.direction);
    }

    #[test]
    fn ray_creation_default() {
        let ray = Ray::default();
        let expected_ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));

        assert_eq!(expected_ray.origin, ray.origin);
        assert_eq!(expected_ray.direction, ray.direction);
    }
}
