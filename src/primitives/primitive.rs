use crate::intersections::Intersections;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector3;
use std::any::Any;
use std::fmt::Debug;

pub trait Primitive: Debug + Any {
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn normal(&self, world: Point) -> Vector3;

    fn as_any(&self) -> &dyn Any;

    fn equals_to(&self, other: &dyn Primitive) -> bool
    where
        Self: Sized + PartialEq + Any,
    {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            *self == *other
        } else {
            false
        }
    }
}
