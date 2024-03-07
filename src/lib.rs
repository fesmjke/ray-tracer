pub mod canvas;
pub mod color;
pub mod float_eq;
pub mod point;
pub mod preset;
pub mod ray;
pub mod utils;
pub mod vector;

pub mod primitives {
    pub use sphere::Sphere;
    mod sphere;
}

pub mod matrices {
    pub use matrix::Matrix;
    pub use matrix2::Matrix2;
    pub use matrix3::Matrix3;
    pub use matrix4::Matrix4;
    mod matrix;
    mod matrix2;
    mod matrix3;
    mod matrix4;
}

pub mod transformations {
    pub use builder::Transformable;
    pub use transformation::{Over, Transform};
    mod builder;
    mod transformation;
}
