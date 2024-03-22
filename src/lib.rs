pub mod camera;
pub mod canvas;
pub mod color;
pub mod constants;
pub mod float_eq;
pub mod material;
pub mod point;
pub mod preset;
pub mod ray;
pub mod render;
pub mod utils;
pub mod vector;
pub mod world;

pub mod patterns {
    pub use checker::CheckerPattern;
    pub use gradient::GradientPattern;
    pub use pattern::{Pattern, PatternType};
    pub use plain::PlainPattern;
    pub use ring::RingPattern;
    pub use stripe::StripePattern;
    pub use texture::TexturePattern;
    mod checker;
    mod gradient;
    mod pattern;
    mod plain;
    mod ring;
    mod stripe;
    mod texture;
}

pub mod primitives {
    pub use primitive::{Primitive, PrimitiveShape};

    pub use plane::Plane;
    pub use sphere::Sphere;
    mod primitive;

    mod plane;
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
