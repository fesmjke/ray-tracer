use crate::hit::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    material: Material,
}

impl Triangle {
    pub fn from(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Self {
        Self {
            v0,
            v1,
            v2,
            material,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let v1v0 = self.v1 - self.v0;
        let v2v0 = self.v2 - self.v0;
        let rov0 = ray.origin() - self.v0;
        let n = Vec3::cross_product(&v1v0, &v2v0);
        let q = Vec3::cross_product(&rov0, &ray.direction());
        let d = 1.0 / Vec3::dot(&ray.direction(), &n);
        let u = d * Vec3::dot(&-q, &v2v0);
        let v = d * Vec3::dot(&q, &v1v0);
        let t = d * Vec3::dot(&-n, &rov0);

        if u < 0.0 || v < 0.0 || (u + v) > 1.0 {
            return false;
        }

        // TODO: outward normal

        hit.point = ray.at(hit.t);
        hit.material = self.material.clone();

        return true;
    }
}
