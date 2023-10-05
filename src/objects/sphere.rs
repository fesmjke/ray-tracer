use crate::hit::{Hit, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Vec3::empty_new(),
            radius: 0.0f32,
        }
    }

    pub fn from(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let oc: Vec3 = ray.origin() - self.center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        let mut root = (-b - sqrt_discriminant) / a;

        if root <= t_min || t_max <= root {
            root = (-b + sqrt_discriminant) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        hit.t = root;
        hit.p = ray.at(hit.t);
        hit.normal = (hit.p - self.center) / self.radius;

        true
    }
}
