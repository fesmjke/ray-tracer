use crate::hit::{Hit, Hitable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    fn new() -> Self {
        Self {
            center: Vec3::empty_new(),
            radius: 0.0f32
        }
    }

    fn from(center : Vec3, radius: f32) -> Self {
        Self {
            center,
            radius
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &Hit) -> bool {
        todo!()
    }
}