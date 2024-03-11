use crate::intersections::Intersections;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector3;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn normal(&self, world: Point) -> Vector3;
}
