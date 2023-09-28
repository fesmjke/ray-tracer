use crate::ray::Ray;
use crate::vec3::{Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool;
}

#[derive(Copy, Clone)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

impl Hit {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            p: Vec3::empty_new(),
            normal: Vec3::empty_new(),
        }
    }
}

pub struct HittableList {
    hittable_list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            hittable_list : Vec::new()
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
        let mut closest = t_max; // replace to f64?

        for object in self.hittable_list.iter() {
            if object.hit(&ray, t_min, t_max, &mut temp_hit) {
                hit_any = true;
                closest = temp_hit.t;
                let _ = std::mem::replace(hit, temp_hit); // look at that : "hit = temp_hit";
            }
        }

        hit_any
    }
}