use crate::intersections::Intersections;
use crate::material::Material;
use crate::matrices::Matrix4;
use crate::point::Point;
use crate::primitives::PrimitiveShape::{CubeShape, PlaneShape, SphereShape, TriangleShape};
use crate::primitives::{Cube, Plane, Sphere, Triangle};
use crate::ray::Ray;
use crate::transformations::Transformable;
use crate::vector::Vector3;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn normal(&self, world: &Point) -> Vector3;
    fn material(&self) -> Material;
    fn transformation(&self) -> &Matrix4;
    fn transformation_invert(&self) -> &Matrix4;
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveShape {
    SphereShape(Sphere),
    PlaneShape(Plane),
    CubeShape(Cube),
    TriangleShape(Triangle),
}

impl Primitive for PrimitiveShape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        match self {
            SphereShape(sphere) => {
                let ray = &ray.transform(&sphere.transformation_inverse);
                sphere.intersect(ray)
            }
            PlaneShape(plane) => {
                let ray = &ray.transform(&plane.transformation_inverse);
                plane.intersect(ray)
            }
            CubeShape(cube) => {
                let ray = &ray.transform(&cube.transformation_inverse);
                cube.intersect(ray)
            }
            TriangleShape(triangle) => {
                let ray = &ray.transform(&triangle.transformation_inverse);
                triangle.intersect(ray)
            }
        }
    }

    fn normal(&self, world: &Point) -> Vector3 {
        match self {
            SphereShape(sphere) => {
                // all shapes need to first convert to the local/object space
                let transformation_inverted = sphere.transformation_inverse;
                let local_point = transformation_inverted * *world;
                let local_normal = sphere.normal(&local_point);
                let world_normal = sphere.transformation_inverse_transpose * local_normal;

                world_normal.normalize()
            }
            PlaneShape(plane) => {
                let transformation_inverted = plane.transformation_inverse;
                let local_point = transformation_inverted * *world;
                let local_normal = plane.normal(&local_point);
                let world_normal = plane.transformation_inverse_transpose * local_normal;

                world_normal.normalize()
            }
            CubeShape(cube) => {
                let transformation_inverted = cube.transformation_inverse;
                let local_point = transformation_inverted * *world;
                let local_normal = cube.normal(&local_point);
                let world_normal = cube.transformation_inverse_transpose * local_normal;

                world_normal.normalize()
            }
            TriangleShape(triangle) => {
                let transformation_inverted = triangle.transformation_inverse;
                let local_point = transformation_inverted * *world;
                let local_normal = triangle.normal(&local_point);
                let world_normal = triangle.transformation_inverse_transpose * local_normal;

                world_normal.normalize()
            }
        }
    }

    fn material(&self) -> Material {
        match self {
            SphereShape(sphere) => sphere.material.clone(),
            PlaneShape(plane) => plane.material.clone(),
            CubeShape(cube) => cube.material.clone(),
            TriangleShape(triangle) => triangle.material.clone(),
        }
    }

    fn transformation(&self) -> &Matrix4 {
        match self {
            SphereShape(sphere) => sphere.transformation(),
            PlaneShape(plane) => plane.transformation(),
            CubeShape(cube) => cube.transformation(),
            TriangleShape(triangle) => triangle.transformation(),
        }
    }

    fn transformation_invert(&self) -> &Matrix4 {
        match self {
            SphereShape(sphere) => sphere.transformation_invert(),
            PlaneShape(plane) => plane.transformation_invert(),
            CubeShape(cube) => cube.transformation_invert(),
            TriangleShape(triangle) => triangle.transformation_invert(),
        }
    }
}

impl PartialEq for PrimitiveShape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SphereShape(sphere_a), SphereShape(sphere_b)) => sphere_a == sphere_b,
            (PlaneShape(plane_a), PlaneShape(plane_b)) => plane_a == plane_b,
            (CubeShape(cube_a), CubeShape(cube_b)) => cube_a == cube_b,
            (TriangleShape(triangle_a), TriangleShape(triangle_b)) => triangle_a == triangle_b,
            _ => false,
        }
    }
}
