pub mod canvas;
pub mod color;
pub mod float_eq;
pub mod material;
pub mod point;
pub mod preset;
pub mod ray;
pub mod utils;
pub mod vector;
pub mod world;

pub mod primitives {
    pub use primitive::{Primitive, PrimitiveShape};

    pub use sphere::Sphere;
    mod primitive;

    mod sphere;
}

pub mod lights {
    pub use point_light::PointLight;
    mod point_light;
}

pub mod intersections {
    pub use intersection::{Intersection, Intersections};
    pub use intersection_details::IntersectionDetails;
    mod intersection;
    mod intersection_details;
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
