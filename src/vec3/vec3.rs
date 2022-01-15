use std::ops;

#[derive(Debug)]
pub struct Vec3{
    e : [f32;3]
}

impl Vec3 {
    pub fn new(e0 : f32,e1 : f32,e2 : f32) -> Vec3 {
        Vec3 {
            e : [e0,e1,e2]
        }
    }
}

impl PartialEq for Vec3{
    fn eq(&self,other : &Self) -> bool {
        self.e[0] == other.e[0] && self.e[1] == other.e[1] && self.e[2] == other.e[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            e : [self.e[0] + _rhs.e[0],
                 self.e[1] + _rhs.e[1],
                 self.e[2] + _rhs.e[2]]
        }
    }
}


#[test]
fn eq_two_vectors(){
    let v1 = Vec3 {e : [2f32,4f32,6f32]};
    let v2 = Vec3 {e : [2f32,4f32,6f32]};

    assert_eq!(v1,v2,"testing that two vec3 are equal");
}
#[test]
fn add_two_vectors(){
    let v1 = Vec3 {e : [2f32,4f32,6f32]};
    let v2 = Vec3 {e : [1f32,3f32,5f32]};

    let output = v1 + v2;

    assert_eq!(output,Vec3{e : [3f32,7f32,11f32]},"testing addition of two vectors");
}
