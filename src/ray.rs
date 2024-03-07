use crate::point::Point;
use crate::vector::Vector3;

// TODO: reference and lifetimes
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    fn new(origin: Point, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, time: f64) -> Point {
        self.origin + self.direction * time
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
    use crate::primitives::Sphere;

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
    fn ray_intersect_sphere_at_two_points() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let intersects = sphere.intersect(&ray);
        let expected_intersects = vec![4.0, 6.0];

        assert_eq!(expected_intersects, intersects);
    }

    #[test]
    fn ray_intersect_sphere_at_one_point() {
        let ray = Ray::new(Point::new(0.0, 1.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let intersects = sphere.intersect(&ray);
        let expected_intersects = vec![5.0, 5.0];

        assert_eq!(expected_intersects, intersects);
    }

    #[test]
    fn ray_originated_inside_intersects_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let intersects = sphere.intersect(&ray);
        let expected_intersects = vec![-1.0, 1.0];

        assert_eq!(expected_intersects, intersects);
    }

    #[test]
    fn ray_behind_sphere() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere::default();
        let intersects = sphere.intersect(&ray);
        let expected_intersects = vec![-6.0, -4.0];

        assert_eq!(expected_intersects, intersects);
    }
}
