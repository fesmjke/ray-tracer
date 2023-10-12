use crate::materials::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool;
}

#[derive(Copy, Clone)]
pub struct Hit {
    pub t: f32,
    pub point: Point3,
    pub material: Box<dyn Material>,
    pub normal: Vec3,
    pub front_face: bool,
}

impl Hit {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            point: Point3::empty_new(),
            normal: Vec3::empty_new(),
            front_face: false,
        }
    }

    pub fn set_hit(&mut self, hit: &Hit) {
        self.point = hit.point;
        self.normal = hit.normal;
        self.t = hit.t;
        self.front_face = hit.front_face;
    }

    pub fn set_front_face(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = if Vec3::dot(&ray.direction(), outward_normal) < 0.0 {
            true
        } else {
            false
        };

        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        }
    }
}

pub struct HittableList {
    hittable_list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittable_list: Vec::new(),
        }
    }

    pub fn attach(&mut self, object: Box<dyn Hittable>) {
        self.hittable_list.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let mut temp_hit = Hit::new();
        let mut hit_any = false;
        let mut closest = t_max;

        for object in self.hittable_list.iter() {
            if object.hit(&ray, t_min, closest, &mut temp_hit) {
                hit_any = true;
                closest = temp_hit.t;
                hit.set_hit(&temp_hit);
            }
        }

        hit_any
    }
}
