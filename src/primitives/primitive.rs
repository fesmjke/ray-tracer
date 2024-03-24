use crate::intersections::Intersections;
use crate::material::Material;
use crate::matrices::{Matrix, Matrix4};
use crate::point::Point;
use crate::primitives::PrimitiveShape::{PlaneShape, SphereShape};
use crate::primitives::{Plane, Sphere};
use crate::ray::Ray;
use crate::transformations::Transformable;
use crate::vector::Vector3;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn normal(&self, world: Point) -> Vector3;
    fn material(&self) -> Material;
    fn transformation(&self) -> &Matrix4;
}

#[derive(Debug, Clone)]
pub enum PrimitiveShape<'a> {
    SphereShape(&'a Sphere),
    PlaneShape(&'a Plane),
}

impl Primitive for PrimitiveShape<'_> {
    fn intersect(&self, ray: &Ray) -> Intersections {
        match self {
            SphereShape(sphere) => {
                let ray = &ray.transform(&sphere.transformation().invert());
                sphere.intersect(ray)
            }
            PlaneShape(plane) => {
                let ray = &ray.transform(&plane.transformation().invert());
                plane.intersect(ray)
            }
        }
    }

    fn normal(&self, world: Point) -> Vector3 {
        match self {
            SphereShape(sphere) => {
                // all shapes need to first convert to the local/object space
                let mut transformation_inverted = sphere.transformation().invert();
                let local_point = transformation_inverted.clone() * world;
                let local_normal = sphere.normal(local_point);
                let world_normal = transformation_inverted.transpose() * local_normal;

                world_normal.normalize()
            }
            PlaneShape(plane) => {
                let mut transformation_inverted = plane.transformation().invert();
                let local_point = transformation_inverted.clone() * world;
                let local_normal = plane.normal(local_point);
                let world_normal = transformation_inverted.transpose() * local_normal;

                world_normal.normalize()
            }
        }
    }

    fn material(&self) -> Material {
        match self {
            SphereShape(sphere) => sphere.material.clone(),
            PlaneShape(plane) => plane.material.clone(),
        }
    }

    fn transformation(&self) -> &Matrix4 {
        match self {
            SphereShape(sphere) => sphere.transformation(),
            PlaneShape(plane) => plane.transformation(),
        }
    }
}

impl PartialEq for PrimitiveShape<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SphereShape(sphere_a), SphereShape(sphere_b)) => sphere_a == sphere_b,
            (PlaneShape(plane_a), PlaneShape(plane_b)) => plane_a == plane_b,
            _ => false,
        }
    }
}
