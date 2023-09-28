use crate::hit::{Hit, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Vec3::empty_new(),
            radius: 0.0f32
        }
    }

    pub fn from(center : Vec3, radius: f32) -> Self {
        Self {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let oc: Vec3 = ray.origin() - self.center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = 2.0 * Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit.t = temp;
                hit.p = ray.at(hit.t);
                hit.normal = (hit.p - self.center) / self.radius;
                return true;
            }
            temp = (-b + (b*b - a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit.t = temp;
                hit.p = ray.at(hit.t);
                hit.normal = (hit.p - self.center) / self.radius;
                return true;
            }
        }

        false
    }
}