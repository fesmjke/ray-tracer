use crate::intersections::Intersections;
use crate::lights::PointLight;
use crate::primitives::{Primitive, PrimitiveShape};
use crate::ray::Ray;

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

    pub fn intersect_objects(&self, ray: &Ray) -> Intersections {
        let mut intersections = Intersections::new();

        self.objects
            .iter()
            .for_each(|object| intersections.merge(object.intersect(ray)));

        // TODO: move sort to intersections
        intersections.sort()
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
    use crate::intersections::{Intersection, Intersections};
    use crate::lights::PointLight;
    use crate::material::Material;
    use crate::point::Point;
    use crate::primitives::PrimitiveShape::SphereShape;
    use crate::primitives::{PrimitiveShape, Sphere};
    use crate::ray::Ray;
    use crate::transformations::Transformable;
    use crate::vector::Vector3;
    use crate::world::World;

    fn simulated_world() -> World {
        let sphere_a = Sphere::default().scale(0.5, 0.5, 0.5).transform();

        let sphere_b = Sphere::default().apply_material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .specular(0.2)
                .diffuse(0.7),
        );

        let light_source =
            PointLight::new(Color::new(1.0, 1.0, 1.0), Point::new(-10.0, 10.0, -10.0));

        let world = World::default()
            .add_object(SphereShape(sphere_a.clone()))
            .add_object(SphereShape(sphere_b.clone()))
            .add_light_source(light_source);

        world
    }

    #[test]
    fn world_creation() {
        let world = World::default();

        assert_eq!(0, world.objects.len());
        assert_eq!(0, world.light_sources.len());
    }

    #[test]
    fn world_with_objects() {
        let world = World::default()
            .add_object(SphereShape(
                Sphere::default().scale(0.5, 0.5, 0.5).transform(),
            ))
            .add_object(SphereShape(
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

        let expected_sphere = SphereShape(Sphere::default().scale(0.5, 0.5, 0.5).transform());

        assert_eq!(expected_sphere, world.objects[0]);

        assert_eq!(2, world.objects.len());
        assert_eq!(1, world.light_sources.len());
    }

    #[test]
    fn world_with_objects_intersections() {
        let sphere_a = Sphere::default().scale(0.5, 0.5, 0.5).transform();

        let sphere_b = Sphere::default().apply_material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .specular(0.2)
                .diffuse(0.7),
        );

        let world = simulated_world();

        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));

        let expected_intersections = Intersections::new().with(vec![
            Intersection::new(4.0, SphereShape(sphere_b.clone())),
            Intersection::new(4.5, SphereShape(sphere_a.clone())),
            Intersection::new(5.5, SphereShape(sphere_a.clone())),
            Intersection::new(6.0, SphereShape(sphere_b.clone())),
        ]);

        assert_eq!(expected_intersections, world.intersect_objects(&ray));
    }
}
