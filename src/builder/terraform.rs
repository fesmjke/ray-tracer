use crate::camera::Camera;
use crate::lights::PointLight;
use crate::primitives::PrimitiveShape;
use crate::world::World;

struct Terraform<'a> {
    world: World<'a>,
}

impl<'a> Terraform<'a> {
    pub fn parse(raw_world: &str) -> Terraform<'a> {
        unimplemented!();

        let mut world = World::default();

        for token in raw_world.split('\n') {
            // todo
        }

        Self { world }
    }

    fn parse_primitive(&self, raw_primitive: &str) -> PrimitiveShape {
        unimplemented!()
    }

    fn parse_light(&self, raw_light: &str) -> PointLight {
        unimplemented!()
    }

    fn parse_camera(&self, raw_camera: &str) -> Camera {
        unimplemented!()
    }
}

#[cfg(test)]
mod terraform_tests {
    use crate::builder::terraform::Terraform;
    use crate::primitives::{PrimitiveShape, Sphere};
    use crate::world::World;

    #[test]
    fn terraform_creation() {
        let raw_world = "";

        let terraform = Terraform::parse(raw_world);
        let expected_world = World::default();

        assert_eq!(expected_world, terraform.world);
    }

    #[test]
    fn terraform_one_primitive() {
        let raw_world = r#"
        Primitive:
          primitive_type: "sphere"
        "#;

        let terraform = Terraform::parse(raw_world);
        let sphere_default = Sphere::default();
        let expected_sphere = PrimitiveShape::SphereShape(&sphere_default);

        assert_eq!(expected_sphere, terraform.world.get_primitive(0));
    }
}
