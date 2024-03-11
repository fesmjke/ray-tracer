use crate::intersections::Intersections;
use crate::material::Material;
use crate::point::Point;
use crate::primitives::PrimitiveShape::SphereShape;
use crate::primitives::Sphere;
use crate::ray::Ray;
use crate::vector::Vector3;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn normal(&self, world: Point) -> Vector3;
    fn material(&self) -> Material;
}

#[derive(Debug, Clone)]
pub enum PrimitiveShape {
    SphereShape(Sphere),
}

impl Primitive for PrimitiveShape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        match self {
            SphereShape(sphere) => sphere.intersect(ray),
        }
    }

    fn normal(&self, world: Point) -> Vector3 {
        match self {
            SphereShape(sphere) => sphere.normal(world),
        }
    }

    fn material(&self) -> Material {
        match self {
            SphereShape(sphere) => sphere.material,
        }
    }
}

impl PartialEq for PrimitiveShape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SphereShape(sphere_a), SphereShape(sphere_b)) => sphere_a == sphere_b,
            // _ => panic!("DIFFERENT TYPE OF SHAPES"),
        }
    }
}
