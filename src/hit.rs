use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub trait Hittable: 'static + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool;
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub point: Point3,
    pub material: Material,
    pub normal: Vec3,
    pub front_face: bool,
}

impl Hit {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            point: Point3::empty_new(),
            material: Material::new(),
            normal: Vec3::empty_new(),
            front_face: bool::default(),
        }
    }

    pub fn set_front_face(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction(), outward_normal) < 0.0;
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
                *hit = temp_hit.clone();
            }
        }

        hit_any
    }
}
