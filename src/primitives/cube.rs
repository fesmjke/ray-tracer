use crate::intersections::Intersections;
use crate::material::Material;
use crate::matrices::{Matrix, Matrix4};
use crate::point::Point;
use crate::primitives::Primitive;
use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    pub transformation: Matrix4,
    pub transformation_inverse: Matrix4,
    pub transformation_inverse_transpose: Matrix4,
    pub material: Material,
}

impl Cube {
    pub fn new(material: Material) -> Self {
        Self {
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            transformation_inverse_transpose: Matrix4::identity(),
            material,
        }
    }

    pub fn apply_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
}

impl Primitive for Cube {
    fn intersect(&self, ray: &Ray) -> Intersections {
        todo!()
    }

    fn normal(&self, world: &Point) -> Vector3 {
        todo!()
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn transformation(&self) -> &Matrix4 {
        &self.transformation
    }

    fn transformation_invert(&self) -> &Matrix4 {
        &self.transformation_inverse
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            material: Material::default(),
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            transformation_inverse_transpose: Matrix4::identity(),
        }
    }
}

#[cfg(test)]
mod cube_tests {
    use crate::material::Material;
    use crate::matrices::{Matrix, Matrix4};
    use crate::primitives::Cube;

    #[test]
    fn cube_creation() {
        let cube = Cube::default();
        let expected_cube = Cube {
            material: Material::default(),
            transformation: Matrix4::identity(),
            transformation_inverse: Matrix4::identity(),
            transformation_inverse_transpose: Matrix4::identity(),
        };

        assert_eq!(expected_cube, cube);
    }
}
