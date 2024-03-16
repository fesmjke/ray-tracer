use crate::intersections::Intersections;
use crate::material::Material;
use crate::point::Point;
use crate::primitives::PrimitiveShape::{PlaneShape, SphereShape};
use crate::primitives::{Plane, Sphere};
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
    PlaneShape(Plane),
}

impl Primitive for PrimitiveShape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        match self {
            SphereShape(sphere) => sphere.intersect(ray),
            PlaneShape(plane) => plane.intersect(ray),
        }
    }

    fn normal(&self, world: Point) -> Vector3 {
        match self {
            SphereShape(sphere) => sphere.normal(world),
            PlaneShape(plane) => plane.normal(world),
        }
    }

    fn material(&self) -> Material {
        match self {
            SphereShape(sphere) => sphere.material,
            PlaneShape(plane) => plane.material,
        }
    }
}

impl PartialEq for PrimitiveShape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SphereShape(sphere_a), SphereShape(sphere_b)) => sphere_a == sphere_b,
            (PlaneShape(plane_a), PlaneShape(plane_b)) => plane_a == plane_b,
            _ => panic!("DIFFERENT TYPE OF SHAPES"),
        }
    }
}
