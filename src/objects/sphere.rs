use crate::hit::{Hit, Hittable};
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::cell::Cell;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<Cell<dyn Material>>,
}

impl Sphere {
    pub fn from(center: Vec3, radius: f32, material: Box<Cell<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            material,
        }
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
        hit.point = ray.at(hit.t);
        let outward_normal = (hit.point - self.center) / self.radius;
        hit.set_front_face(ray, &outward_normal);
        hit.material = Box::from(self.material.as_ptr().clone());

        true
    }
}
