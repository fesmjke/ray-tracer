use crate::point::Point;
use crate::ray::Ray;

pub struct Sphere {
    id: String, // TODO: replace with UUID
    pub origin: Point,
    pub radius: f64,
}

impl Sphere {
    fn new(id: String, origin: Point, radius: f64) -> Self {
        Self { id, origin, radius }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - Point::default();
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot_point(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - (4.0 * a * c);

        if discriminant < 0.0 {
            vec![]
        } else {
            let x1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let x2 = (-b + discriminant.sqrt()) / (2.0 * a);

            vec![x1, x2]
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            id: String::default(), // TODO: replace with UUID (not unique).
            origin: Point::default(),
            radius: 1.0,
        }
    }
}

#[cfg(test)]
mod sphere_tests {
    use crate::point::Point;
    use crate::primitives::sphere::Sphere;

    #[test]
    fn sphere_creation() {
        let sphere = Sphere::new(String::from("Sphere"), Point::new(0.0, 0.0, 0.0), 1.0);
        let expected_origin = Point::default();
        let expected_radius = 1.0;

        assert_eq!(expected_origin, sphere.origin);
        assert_eq!(expected_radius, sphere.radius);
    }

    #[test]
    fn sphere_creation_default() {
        let sphere = Sphere::default();
        let expected_origin = Point::default();
        let expected_radius = 1.0;

        assert_eq!(expected_origin, sphere.origin);
        assert_eq!(expected_radius, sphere.radius);
    }
}
