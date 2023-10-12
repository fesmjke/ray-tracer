use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec3::Color;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit: &Hit,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool;
}
