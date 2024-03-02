use crate::matrices::{Matrix, Matrix4};

pub enum Transform {
    Translate(f64, f64, f64),
    Rotate,
    Scale,
    Shear,
}

impl Transform {
    pub fn transformation(self) -> Matrix4 {
        match self {
            Transform::Translate(x, y, z) => {
                Matrix4::identity().set(0, 3, x).set(1, 3, y).set(2, 3, z)
            }
            Transform::Rotate => {
                todo!()
            }
            Transform::Scale => {
                todo!()
            }
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
}
