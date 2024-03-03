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
    fn transform(self) -> Matrix4 {
        self.inner.transform(&self.transformation)
    }

    fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        self.transformation = Translate(x, y, z).transformation() * self.transformation;
        self
    }

    fn rotate(mut self, over: Over, angle: f64) -> Self {
        self.transformation = Rotate(over, angle).transformation() * self.transformation;
        self
    }

    fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.transformation = Scale(x, y, z).transformation() * self.transformation;
        self
    }

    fn shear(mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        self.transformation = Shear(xy, xz, yx, yz, zx, zy).transformation() * self.transformation;
        self
    }
}

pub trait Transformable {
    fn transform(self, transformation: &Matrix4) -> Matrix4;
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
    use crate::transformations::builder::TransformBuilder;

    #[test]
    fn builder_point() {}
}
