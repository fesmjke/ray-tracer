use crate::matrices::Matrix4;
use crate::transformations::transformation::Over;
use crate::transformations::transformation::Transform::{Rotate, Scale, Shear, Translate};

pub struct TransformBuilder<T> {
    pub transformation: Matrix4,
    pub inner: T,
}

impl<T> TransformBuilder<T>
where
    T: Transformable,
{
    pub fn transform(self) -> T {
        self.inner.transform(&self.transformation)
    }

    pub fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        self.transformation = Translate(x, y, z).transformation() * self.transformation;
        self
    }

    pub fn rotate(mut self, over: Over, angle: f64) -> Self {
        self.transformation = Rotate(over, angle).transformation() * self.transformation;
        self
    }

    pub fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.transformation = Scale(x, y, z).transformation() * self.transformation;
        self
    }

    pub fn shear(mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        self.transformation = Shear(xy, xz, yx, yz, zx, zy).transformation() * self.transformation;
        self
    }
}

pub trait Transformable {
    fn transform(self, transformation: &Matrix4) -> Self;
    fn translate(self, x: f64, y: f64, z: f64) -> TransformBuilder<Self>
    where
        Self: Sized,
    {
        TransformBuilder {
            transformation: Translate(x, y, z).transformation(),
            inner: self,
        }
    }
    fn rotate(self, over: Over, angle: f64) -> TransformBuilder<Self>
    where
        Self: Sized,
    {
        TransformBuilder {
            transformation: Rotate(over, angle).transformation(),
            inner: self,
        }
    }
    fn scale(self, x: f64, y: f64, z: f64) -> TransformBuilder<Self>
    where
        Self: Sized,
    {
        TransformBuilder {
            transformation: Scale(x, y, z).transformation(),
            inner: self,
        }
    }
    fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> TransformBuilder<Self>
    where
        Self: Sized,
    {
        TransformBuilder {
            transformation: Shear(xy, xz, yx, yz, zx, zy).transformation(),
            inner: self,
        }
    }
}

#[cfg(test)]
mod transformations_builder_tests {
    use crate::point::Point;
    use crate::transformations::transformation::Over;
    use crate::transformations::Transformable;
    use crate::vector::Vector3;
    use std::f64::consts::PI;

    #[test]
    fn builder_point() {
        let point = Point::new(1.0, 0.0, 1.0)
            .rotate(Over::X, PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0)
            .transform();

        let expected_point = Point::new(15.0, 0.0, 7.0);

        assert_eq!(expected_point, point);
    }

    #[test]
    fn builder_vector() {
        let vector = Vector3::new(0.0, 1.0, 0.0)
            .rotate(Over::X, PI / 2.0)
            .transform()
            .scale(0.0, 0.0, 10.0)
            .transform();

        let expected_vector = Vector3::new(0.0, 0.0, 10.0);

        assert_eq!(expected_vector, vector);
    }
}
