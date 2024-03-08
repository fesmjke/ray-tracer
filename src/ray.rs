use crate::matrices::Matrix4;
use crate::point::Point;
use crate::transformations::Transformable;
use crate::vector::Vector3;

// TODO: reference and lifetimes
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, time: f64) -> Point {
        self.origin + self.direction * time
    }
}

impl Transformable for Ray {
    fn transform(self, transformation: &Matrix4) -> Ray {
        Ray {
            // TODO: Replace with {*transformation} when matrix is slice and has Copy trait, instead of vector
            origin: transformation.clone() * self.origin,
            direction: transformation.clone() * self.direction,
        }
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

    #[test]
    fn ray_position_over_time() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector3::new(1.0, 0.0, 0.0));

        assert_eq!(Point::new(2.0, 3.0, 4.0), ray.position(0.0));
        assert_eq!(Point::new(3.0, 3.0, 4.0), ray.position(1.0));
        assert_eq!(Point::new(1.0, 3.0, 4.0), ray.position(-1.0));
        assert_eq!(Point::new(4.5, 3.0, 4.0), ray.position(2.5));
    }

    #[test]
    fn ray_translate() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector3::new(0.0, 1.0, 0.0));
        let expected_ray = Ray::new(Point::new(4.0, 6.0, 8.0), Vector3::new(0.0, 1.0, 0.0));

        let translated_ray = ray.translate(3.0, 4.0, 5.0).transform();

        assert_eq!(expected_ray.origin, translated_ray.origin);
        assert_eq!(expected_ray.direction, translated_ray.direction);
    }

    #[test]
    fn ray_scale() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector3::new(0.0, 1.0, 0.0));
        let expected_ray = Ray::new(Point::new(2.0, 6.0, 12.0), Vector3::new(0.0, 3.0, 0.0));

        let translated_ray = ray.scale(2.0, 3.0, 4.0).transform();

        assert_eq!(expected_ray.origin, translated_ray.origin);
        assert_eq!(expected_ray.direction, translated_ray.direction);
    }
}
