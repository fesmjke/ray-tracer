use crate::float_eq::ApproxEq;
use crate::intersections::{Intersection, Intersections};
use crate::material::Material;
use crate::matrices::{Matrix, Matrix4};
use crate::point::Point;
use crate::primitives::Primitive;
use crate::primitives::PrimitiveShape::CubeShape;
use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    pub transformation: Matrix4,
    pub transformation_inverse: Matrix4,
    pub transformation_inverse_transpose: Matrix4,
    pub material: Material,
}

impl Cube {
    pub fn new(material: Material) -> Self {
        Self {
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            transformation_inverse_transpose: Matrix4::identity(),
            material,
        }
    }

    pub fn apply_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

impl Primitive for Cube {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let intersections = Intersections::new();
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z, ray.direction.z);

        let tmin = vec![xtmin, ytmin, ztmin]
            .into_iter()
            .reduce(f64::max)
            .unwrap_or(0.0);
        let tmax = vec![xtmax, ytmax, ztmax]
            .into_iter()
            .reduce(f64::min)
            .unwrap_or(0.0);

        if tmin > tmax {
            return intersections;
        }

        intersections.with(vec![
            Intersection::new(tmin, CubeShape(self)),
            Intersection::new(tmax, CubeShape(self)),
        ])
    }
    fn normal(&self, world: &Point) -> Vector3 {
        let maxc = vec![world.x.abs(), world.y.abs(), world.z.abs()]
            .into_iter()
            .reduce(f64::max)
            .unwrap_or_default();

        return if maxc.approx_eq_low(&world.x.abs()) {
            Vector3::new(world.x, 0.0, 0.0)
        } else if maxc.approx_eq_low(&world.y.abs()) {
            Vector3::new(0.0, world.y, 0.0)
        } else {
            Vector3::new(0.0, 0.0, world.z)
        };
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn transformation(&self) -> &Matrix4 {
        &self.transformation
    }

    fn transformation_invert(&self) -> &Matrix4 {
        &self.transformation_inverse
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            material: Material::default(),
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            transformation_inverse_transpose: Matrix4::identity(),
        }
    }
}

#[cfg(test)]
mod cube_tests {
    use crate::intersections::Intersections;
    use crate::material::Material;
    use crate::matrices::{Matrix, Matrix4};
    use crate::point::Point;
    use crate::primitives::{Cube, Primitive};
    use crate::ray::Ray;
    use crate::vector::Vector3;

    #[test]
    fn cube_creation() {
        let cube = Cube::default();
        let expected_cube = Cube {
            material: Material::default(),
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            transformation_inverse_transpose: Matrix4::identity(),
        };

        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn cube_ray_intersection() {
        let cube = Cube::default();

        let ray_plus_x = Ray::new(Point::new(5.0, 0.5, 0.0), Vector3::new(-1.0, 0.0, 0.0));
        let intersections = cube.intersect(&ray_plus_x);
        let expected_time = (4.0, 6.0);
        assert_eq!(
            expected_time,
            (
                intersections.intersections[0].time,
                intersections.intersections[1].time
            )
        );

        let ray_minus_x = Ray::new(Point::new(-5.0, 0.5, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let intersections = cube.intersect(&ray_minus_x);
        let expected_time = (4.0, 6.0);
        assert_eq!(
            expected_time,
            (
                intersections.intersections[0].time,
                intersections.intersections[1].time
            )
        );

        let ray_plus_y = Ray::new(Point::new(0.5, 5.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        let intersections = cube.intersect(&ray_plus_y);
        let expected_time = (4.0, 6.0);
        assert_eq!(
            expected_time,
            (
                intersections.intersections[0].time,
                intersections.intersections[1].time
            )
        );
        let ray_minus_y = Ray::new(Point::new(0.5, -5.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let intersections = cube.intersect(&ray_minus_y);
        let expected_time = (4.0, 6.0);
        assert_eq!(
            expected_time,
            (
                intersections.intersections[0].time,
                intersections.intersections[1].time
            )
        );

        let ray_plus_z = Ray::new(Point::new(0.5, 0.0, 5.0), Vector3::new(0.0, 0.0, -1.0));
        let intersections = cube.intersect(&ray_plus_z);
        let expected_time = (4.0, 6.0);
        assert_eq!(
            expected_time,
            (
                intersections.intersections[0].time,
                intersections.intersections[1].time
            )
        );
        let ray_minus_z = Ray::new(Point::new(0.5, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let intersections = cube.intersect(&ray_minus_z);
        let expected_time = (4.0, 6.0);
        assert_eq!(
            expected_time,
            (
                intersections.intersections[0].time,
                intersections.intersections[1].time
            )
        );

        let ray_inside = Ray::new(Point::new(0.0, 0.5, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let intersections = cube.intersect(&ray_inside);
        let expected_time = (-1.0, 1.0);
        assert_eq!(
            expected_time,
            (
                intersections.intersections[0].time,
                intersections.intersections[1].time
            )
        );
    }

    #[test]
    fn cube_ray_miss_intersection() {
        let cube = Cube::default();

        let ray = Ray::new(
            Point::new(-2.0, 0.0, 0.0),
            Vector3::new(0.2673, 0.5345, 0.8018),
        );
        let intersections = cube.intersect(&ray);
        let expected_intersections = Intersections::new();
        assert_eq!(expected_intersections, intersections);

        let ray = Ray::new(
            Point::new(0.0, -2.0, 0.0),
            Vector3::new(0.8018, 0.2673, 0.5345),
        );
        let intersections = cube.intersect(&ray);
        let expected_intersections = Intersections::new();
        assert_eq!(expected_intersections, intersections);

        let ray = Ray::new(
            Point::new(0.0, 0.0, -2.0),
            Vector3::new(0.5345, 0.8018, 0.2673),
        );
        let intersections = cube.intersect(&ray);
        let expected_intersections = Intersections::new();
        assert_eq!(expected_intersections, intersections);

        let ray = Ray::new(Point::new(2.0, 0.0, 2.0), Vector3::new(0.0, 0.0, -1.0));
        let intersections = cube.intersect(&ray);
        let expected_intersections = Intersections::new();
        assert_eq!(expected_intersections, intersections);

        let ray = Ray::new(Point::new(0.0, 2.0, 2.0), Vector3::new(0.0, -1.0, 0.0));
        let intersections = cube.intersect(&ray);
        let expected_intersections = Intersections::new();
        assert_eq!(expected_intersections, intersections);

        let ray = Ray::new(Point::new(2.0, 2.0, 0.0), Vector3::new(-1.0, 0.0, 0.0));
        let intersections = cube.intersect(&ray);
        let expected_intersections = Intersections::new();
        assert_eq!(expected_intersections, intersections);
    }

    #[test]
    fn cube_normal_on_surface() {
        let cube = Cube::default();
        let point = Point::new(1.0, 0.5, -0.8);
        let vector = cube.normal(&point);
        let expected_vector = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(expected_vector, vector);

        let point = Point::new(-1.0, -0.2, 0.9);
        let vector = cube.normal(&point);
        let expected_vector = Vector3::new(-1.0, 0.0, 0.0);

        assert_eq!(expected_vector, vector);

        let point = Point::new(-0.4, 1.0, -0.1);
        let vector = cube.normal(&point);
        let expected_vector = Vector3::new(0.0, 1.0, 0.0);

        assert_eq!(expected_vector, vector);

        let point = Point::new(0.3, -1.0, -0.7);
        let vector = cube.normal(&point);
        let expected_vector = Vector3::new(0.0, -1.0, 0.0);

        assert_eq!(expected_vector, vector);

        let point = Point::new(-0.6, 0.3, 1.0);
        let vector = cube.normal(&point);
        let expected_vector = Vector3::new(0.0, 0.0, 1.0);

        assert_eq!(expected_vector, vector);

        let point = Point::new(0.4, 0.4, -1.0);
        let vector = cube.normal(&point);
        let expected_vector = Vector3::new(0.0, 0.0, -1.0);

        assert_eq!(expected_vector, vector);

        let point = Point::new(1.0, 1.0, 1.0);
        let vector = cube.normal(&point);
        let expected_vector = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(expected_vector, vector);

        let point = Point::new(-1.0, -1.0, -1.0);
        let vector = cube.normal(&point);
        let expected_vector = Vector3::new(-1.0, 0.0, 0.0);

        assert_eq!(expected_vector, vector);
    }
}
