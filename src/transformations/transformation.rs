use crate::matrices::{Matrix, Matrix4};

// TODO: find better solution, or union with transformation?
pub enum Over {
    X,
    Y,
    Z,
}

pub enum Transform {
    Translate(f64, f64, f64),
    Rotate(Over, f64),
    Scale(f64, f64, f64),
    Shear,
}

impl Transform {
    pub fn transformation(self) -> Matrix4 {
        match self {
            Transform::Translate(x, y, z) => {
                Matrix4::identity().set(0, 3, x).set(1, 3, y).set(2, 3, z)
            }
            Transform::Rotate(over, angle) => match over {
                Over::X => Matrix4::identity()
                    .set(1, 1, f64::cos(angle))
                    .set(1, 2, -f64::sin(angle))
                    .set(2, 1, f64::sin(angle))
                    .set(2, 2, f64::cos(angle)),
                Over::Y => Matrix4::identity()
                    .set(0, 0, f64::cos(angle))
                    .set(0, 2, f64::sin(angle))
                    .set(2, 0, -f64::sin(angle))
                    .set(2, 2, f64::cos(angle)),
                Over::Z => {
                    todo!()
                }
            },
            Transform::Scale(x, y, z) => Matrix4::identity().set(0, 0, x).set(1, 1, y).set(2, 2, z),
            Transform::Shear => {
                todo!()
            }
        }
    }
}

/// TODO
pub trait Transferable {
    fn translate(self, x: f64, y: f64, z: f64);
}

#[cfg(test)]
mod transformations_tests {
    use super::*;
    use crate::point::Point;
    use crate::vector::Vector3;
    use std::f64::consts::PI;

    #[test]
    fn transformation_transition_vector() {
        let transformation = Transform::Translate(5.0, -3.0, 2.0).transformation();
        let vector = Vector3::new(-3.0, 4.0, 5.0);
        let expected_point = Vector3::new(-3.0, 4.0, 5.0);

        let nvector = transformation * vector;

        assert_eq!(expected_point, nvector);
    }

    #[test]
    fn transformation_transition_point() {
        let transformation = Transform::Translate(5.0, -3.0, 2.0).transformation();
        let point = Point::new(-3.0, 4.0, 5.0);
        let expected_point = Point::new(2.0, 1.0, 7.0);

        let npoint = transformation * point;

        assert_eq!(expected_point, npoint);
    }

    #[test]
    fn transformation_transition_inverse_point() {
        let transformation = Transform::Translate(5.0, -3.0, 2.0).transformation();
        let point = Point::new(-3.0, 4.0, 5.0);
        let expected_point = Point::new(-8.0, 7.0, 3.0);

        let npoint = transformation.invert() * point;

        assert_eq!(expected_point, npoint);
    }

    #[test]
    fn transformation_scaling_point() {
        let transformation = Transform::Scale(2.0, 3.0, 4.0).transformation();
        let point = Point::new(-4.0, 6.0, 8.0);
        let expected_point = Point::new(-8.0, 18.0, 32.0);

        let npoint = transformation * point;

        assert_eq!(expected_point, npoint);
    }

    #[test]
    fn transformation_scaling_vector() {
        let transformation = Transform::Scale(2.0, 3.0, 4.0).transformation();
        let vector = Vector3::new(-4.0, 6.0, 8.0);
        let expected_vector = Vector3::new(-8.0, 18.0, 32.0);

        let nvector = transformation * vector;

        assert_eq!(expected_vector, nvector);
        assert_eq!(expected_vector.magnitude(), nvector.magnitude()); // ~37.5765
    }

    #[test]
    fn transformation_scaling_inverse_vector() {
        let transformation = Transform::Scale(2.0, 3.0, 4.0).transformation();
        let vector = Vector3::new(-4.0, 6.0, 8.0);
        let expected_vector = Vector3::new(-2.0, 2.0, 2.0);

        let nvector = transformation.invert() * vector;

        assert_eq!(expected_vector, nvector);
    }

    #[test]
    fn transformation_scaling_reflect_vector() {
        // TODO: later add separate method for reflection in different axis
        let transformation = Transform::Scale(-1.0, 1.0, 1.0).transformation();
        let vector = Vector3::new(2.0, 3.0, 4.0);
        let expected_vector = Vector3::new(-2.0, 3.0, 4.0);

        let nvector = transformation * vector;

        assert_eq!(expected_vector, nvector);
    }

    #[test]
    fn transformation_rotate_over_x_point() {
        // TODO: later add separate method for reflection in different axis
        let half_quarter = Transform::Rotate(Over::X, PI / 4.0).transformation();
        let full_quarter = Transform::Rotate(Over::X, PI / 2.0).transformation();
        let point = Point::new(0.0, 1.0, 0.0);
        let expected_point_half = Point::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0);
        let expected_point_full = Point::new(0.0, 0.0, 1.0);

        let npoint_half = half_quarter * point;
        let npoint_full = full_quarter * point;

        assert_eq!(expected_point_half, npoint_half);
        assert_eq!(expected_point_full, npoint_full);
    }

    #[test]
    fn transformation_rotate_over_x_inverse_point() {
        // TODO: later add separate method for reflection in different axis
        let half_quarter = Transform::Rotate(Over::X, PI / 4.0).transformation();

        let point = Point::new(0.0, 1.0, 0.0);
        let expected_point_half = Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);

        let npoint_half = half_quarter.invert() * point;

        assert_eq!(expected_point_half, npoint_half);
    }

    #[test]
    fn transformation_rotate_over_y_point() {
        // TODO: later add separate method for reflection in different axis
        let half_quarter = Transform::Rotate(Over::Y, PI / 4.0).transformation();
        let full_quarter = Transform::Rotate(Over::Y, PI / 2.0).transformation();
        let point = Point::new(0.0, 0.0, 1.0);
        let expected_point_half = Point::new(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0);
        let expected_point_full = Point::new(1.0, 0.0, 0.0);

        let npoint_half = half_quarter * point;
        let npoint_full = full_quarter * point;

        assert_eq!(expected_point_half, npoint_half);
        assert_eq!(expected_point_full, npoint_full);
    }
}
