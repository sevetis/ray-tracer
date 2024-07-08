
pub type Color = Vec3;
pub type Point = Vec3;

pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 {
            e: [e1, e2, e3]
        }
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
}