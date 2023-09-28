use crate::ray::Ray;
use crate::vec3::{Vec3};

pub struct Hit {
    t: f32,
    p: Vec3,
    normal: Vec3
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &Hit) -> bool;
}