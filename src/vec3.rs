use std::ops::{Index, IndexMut, Add, Sub, Mul, Div};
use std::fmt;
use rand::Rng;

const DIMENSION: usize = 3;

pub type Point = Vec3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    e: [f64; DIMENSION]
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= DIMENSION {
            panic!("Index {} out of range", index);
        }    
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        if index >= DIMENSION {
            panic!("Index {} out of range", index);
        }
        &mut self.e[index]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl Vec3 {
    pub const fn new(e: [f64; DIMENSION]) -> Vec3 {
        Vec3 { e: e }
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut result = Vec3::new([0.0; DIMENSION]);
        let mut rng = rand::thread_rng();
        for i in 0..DIMENSION {
            result[i] = rng.gen_range(min..max);
        }
        result
    }
    
    pub fn random_in_unit_sphere() -> Vec3{
        let mut result: Vec3;
        loop {
            result = Vec3::random(-1.0, 1.0);
            if result.square() < 1.0 { break; }
        }
        result
    }

    pub fn random_unit_vec() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }
            
    pub fn x(&self) -> f64 { self[0] }
    pub fn y(&self) -> f64 { self[1] }
    pub fn z(&self) -> f64 { self[2] }

    pub fn reverse(&self) ->Vec3 {
        let mut e = [0.0; DIMENSION];
        for i in 0..DIMENSION {
            e[i] = -self[i];
        }
        Vec3 { e: e }
    }

    pub fn square(&self) -> f64 {
        let mut res: f64 = 0.0;
        for i in 0..DIMENSION {
            res += self[i] * self[i];
        }
        res
    }

    pub fn length(&self) -> f64 {
        self.square().sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        let mut result = 0.0;
        for i in 0..DIMENSION {
            result += self[i] * rhs[i];
        }
        result
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        let mut e: [f64; DIMENSION] = [0.0; DIMENSION];
        for i in 0..(DIMENSION - 2) {
            e[i] = self[i + 1] * rhs[i + 2] - self[i + 2] * rhs[i + 1];
        }
        e[DIMENSION - 2] = self[DIMENSION - 1] * rhs[0] - self[0] * rhs[DIMENSION - 1];
        e[DIMENSION - 1] = self[0] * rhs[1] - self[1] * rhs[0];
        Vec3 { e: e }
    }

    pub fn near_zero(&self) -> bool {
        let mut result = true;
        let threshold = 1e-8;
        for i in 0..DIMENSION {
            result &= self[i] < threshold;
            if !result { break; }
        }
        result
    }

    pub fn specular(&self, normal: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * (*normal)
    }

    pub fn refract(&self, normal: &Vec3, eta_ratio: f64) -> Vec3 {
        let cos_theta = normal.dot(&self.reverse()).min(1.0);
        let perpendicular = eta_ratio * (*self + (*normal) * cos_theta);
        let parallel = -((1.0 - perpendicular.square()).abs().sqrt()) * (*normal);
        perpendicular + parallel
    }
}


impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        let mut e: [f64; DIMENSION] = [0.0; DIMENSION];
        for i in 0..DIMENSION {
            e[i] = self[i] + rhs[i];
        }
        Vec3 { e: e }
    }
}
        
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self + rhs.reverse()
    }
}



impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut e = [0.0; DIMENSION];
        for i in 0..DIMENSION {
            e[i] = self[i] * rhs[i];
        }
        Vec3 { e: e }
    }
}


impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Self::Output {
        let mut e = [0.0; DIMENSION];
        for i in 0..DIMENSION {
            e[i] = self[i] + rhs;
        }
        Vec3 { e: e }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut e = [0.0; DIMENSION];
        for i in 0..DIMENSION {
            e[i] = self[i] * rhs;
        }
        Vec3 { e: e }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

