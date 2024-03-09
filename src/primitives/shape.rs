use crate::point::Point;
use crate::vector::Vector3;

pub trait Shape {
    fn normal(&self, world: Point) -> Vector3;
}
