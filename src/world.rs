use crate::lights::PointLight;
use crate::primitives::PrimitiveShape;

pub struct World {
    objects: Vec<PrimitiveShape>,
    light_sources: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::with_capacity(8),
            light_sources: Vec::with_capacity(4),
        }
    }

    pub fn add_object(mut self, object: PrimitiveShape) -> Self {
        self.objects.push(object);
        self
    }

    pub fn add_light_source(mut self, source: PointLight) -> Self {
        self.light_sources.push(source);
        self
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod world_tests {
    use crate::color::Color;
    use crate::lights::PointLight;
    use crate::material::Material;
    use crate::point::Point;
    use crate::primitives::{PrimitiveShape, Sphere};
    use crate::transformations::Transformable;
    use crate::world::World;

    #[test]
    fn world_creation() {
        let world = World::default();

        assert_eq!(0, world.objects.len());
        assert_eq!(0, world.light_sources.len());
    }

    #[test]
    fn world_with_objects() {
        let world = World::default()
            .add_object(PrimitiveShape::SphereShape(
                Sphere::default().scale(0.5, 0.5, 0.5).transform(),
            ))
            .add_object(PrimitiveShape::SphereShape(
                Sphere::default().apply_material(
                    Material::default()
                        .color(Color::new(0.8, 1.0, 0.6))
                        .specular(0.2)
                        .diffuse(0.7),
                ),
            ))
            .add_light_source(PointLight::new(
                Color::new(1.0, 1.0, 1.0),
                Point::new(-10.0, 10.0, -10.0),
            ));

        let expected_sphere =
            PrimitiveShape::SphereShape(Sphere::default().scale(0.5, 0.5, 0.5).transform());

        assert_eq!(expected_sphere, world.objects[0]);

        assert_eq!(2, world.objects.len());
        assert_eq!(1, world.light_sources.len());
    }
}
