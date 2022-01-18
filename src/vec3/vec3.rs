use std::ops;

#[derive(Debug,Copy,Clone)]
pub struct Vec3{
    e : [f32;3]
}

impl Vec3 {
    pub fn new(e0 : f32,e1 : f32,e2 : f32) -> Vec3 {
        Vec3 {
            e : [e0,e1,e2]
        }
    }
    pub fn empty_new() -> Vec3{
        Vec3 {
            e : [0f32,0f32,0f32]
        }
    }
    pub fn negative_vec(&self) -> Vec3 {
        Vec3{
            e : [-self.e[0],-self.e[1],-self.e[2]]
        }
    }
    pub fn x(self) -> f32 {
        self.e[0]
    }
    pub fn y(self) -> f32 {
        self.e[1]
    }
    pub fn z(self) -> f32 {
        self.e[2]
    }

    pub fn r(self) -> f32 {self.e[0]}
    pub fn g(self) -> f32 {self.e[1]}
    pub fn b(self) -> f32 {self.e[2]}

    pub fn dot(vec_f : &Vec3,vec_s : &Vec3) -> f32 {
        vec_f.e[0] * vec_s.e[0] + vec_f.e[1] * vec_s.e[1] + vec_f.e[2] * vec_s.e[2]
    }

    pub fn cross_product(vec_f : &Vec3,vec_s : &Vec3) -> Vec3{
        Vec3 {
            e : [vec_f.e[1] * vec_s.e[2] - vec_f.e[2] * vec_s.e[1],
                 vec_f.e[2] * vec_s.e[0] - vec_f.e[0] * vec_s.e[2],
                 vec_f.e[0] * vec_s.e[1] - vec_f.e[1] * vec_s.e[0]]
        }
    }

    pub fn length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn unit_vector(v : &Vec3) -> Vec3{
        *v / v.length()
    }
}

impl PartialEq for Vec3{
    fn eq(&self,other : &Self) -> bool {
        self.e[0] == other.e[0] && self.e[1] == other.e[1] && self.e[2] == other.e[2]
    }
}

impl ops::Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self {
        Vec3{
            e : [-self.e[0],-self.e[1],-self.e[2]]
        }
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

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self,_rhs : Self) -> Self {
        Self {
            e : [self.e[0] - _rhs.e[0],
                 self.e[1] - _rhs.e[1],
                 self.e[2] - _rhs.e[2]]
        }
    }
}

impl ops::Mul for Vec3{
    type Output = Self;

    fn mul(self,_rhs: Self) -> Self {
        Self {
            e : [self.e[0] * _rhs.e[0],
                 self.e[1] * _rhs.e[1],
                 self.e[2] * _rhs.e[2]]
        }
    }
}

impl ops::Mul<f32> for Vec3{
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Self {
        Self {
            e : [self.e[0] * _rhs,
                 self.e[1] * _rhs,
                 self.e[2] * _rhs]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: f32) -> Self::Output {
        let k = 1.0/_rhs;

        Self {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    #[test]
    fn sub_two_vectors(){
        let v1 = Vec3 {e : [2f32,4f32,6f32]};
        let v2 = Vec3 {e : [1f32,3f32,5f32]};

        let output = v1 - v2;

        assert_eq!(output,Vec3{e : [1f32,1f32,1f32]});
    }
    #[test]
    fn mul_two_vectors(){
        let v1 = Vec3 {e : [2f32,4f32,6f32]};
        let v2 = Vec3 {e : [1f32,3f32,5f32]};

        let output = v1 * v2;

        assert_eq!(output,Vec3{e : [2f32,12f32,30f32]});
    }
    #[test]
    fn mul_vector_by_value(){
        let v1 = Vec3 {e : [2f32,4f32,6f32]};

        let output = v1 * 2f32;

        assert_eq!(output,Vec3{e : [4f32,8f32,12f32]});
    }
    #[test]
    fn div_vector_by_value(){
        let v1 = Vec3 {e : [2f32,4f32,6f32]};

        let output = v1 / 2.0;

        assert_eq!(output,Vec3 { e : [1f32,2f32,3f32]})
    }
    #[test]
    fn empty_vector(){
        let v1 = Vec3::empty_new();

        assert_eq!(v1,Vec3{e : [0f32,0f32,0f32]});
    }
    #[test]
    fn negative_vec(){
        let v1 = Vec3::new(1f32,2f32,3f32);
        assert_eq!(-v1,Vec3{e : [-1f32,-2f32,-3f32]})
    }
    #[test]
    fn vector_dot(){
        let v1 = Vec3 {e : [2f32,4f32,6f32]};
        let v2 = Vec3 {e : [1f32,3f32,5f32]};

        let dot = Vec3::dot(&v1,&v2);

        assert_eq!(dot,44f32);
    }
    #[test]
    fn vector_length(){
        let v1 = Vec3 {e : [0f32,0f32,5f32]};

        let length = v1.length();

        assert_eq!(length,5.0)
    }
    #[test]
    fn cross_product_of_two_vectors(){
        let v1 = Vec3 {e : [1f32,3f32,4f32]};
        let v2 = Vec3 {e : [2f32,7f32,-5f32]};

        let cross = Vec3::cross_product(&v1,&v2);

        assert_eq!(cross,Vec3{e : [-43f32,13f32,1f32]});
    }
    #[test]
    fn unit_vec(){
        let v1 = Vec3{e : [1f32,3f32,4f32]};

        let unit = Vec3::unit_vector(&v1);

        assert_eq!(unit,Vec3{e : [0.19611613, 0.5883484, 0.78446454]});
    }
}