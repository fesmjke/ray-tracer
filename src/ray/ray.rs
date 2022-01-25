use crate::vec3::vec3::{Point3,Vec3};

#[derive(PartialEq,Debug)]
pub struct Ray{
    origin : Point3,
    direction : Vec3
}

impl Ray {
    pub fn ray(orig : Point3,dir : Vec3) -> Ray {
        Ray {
            origin : orig,
            direction : dir
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3{
        self.direction
    }

    pub fn at(&self,t : f32) -> Point3 {
        return self.origin + self.direction * t;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn ray_init(){
        let p = Point3::new(0f32,0f32,0f32);
        let v = Vec3::new(1f32,2f32,3f32);

        let r = Ray::ray(p,v);

        assert_eq!(r,Ray{direction : Vec3::new(1f32,2f32,3f32),origin : Point3::empty_new()});
    }

    #[test]
    fn ray_at(){
        let p = Point3::new(0f32,0f32,0f32);
        let v = Vec3::new(1f32,2f32,3f32);

        let r = Ray::ray(p,v);

        let p_at = r.at(4.0);

        assert_eq!(p_at,Point3::new(4f32,8f32,12f32));
    }
}