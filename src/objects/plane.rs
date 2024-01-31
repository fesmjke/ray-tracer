use crate::hit::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Plane {
    center: Point3,
    surface_normal: Vec3,
    material: Material,
}

impl Plane {
    pub fn from(center: Point3, surface: Vec3, material: Material) -> Self {
        Self {
            center,
            surface_normal: surface,
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let dot = Vec3::dot(&self.surface_normal, &ray.direction());

        if dot < 0.00001 {
            let p0r0 = self.center - ray.origin();
            let t = Vec3::dot(&p0r0, &self.surface_normal) / dot;
            if t > 0.00001 && t < t_max && t > t_min {
                hit.point = ray.at(hit.t);
                hit.material = self.material.clone();
                // hit.set_front_face(ray, &self.surface_normal);

                return true;
            }
        }

        false
    }
}
