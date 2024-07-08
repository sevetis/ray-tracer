use std::ops::{Index, IndexMut, Add, Sub, Mul, Div};

const DIMENSION: usize = 3;
pub type Color = Vec3;
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

impl Vec3 {
    pub fn new(e: [f64; DIMENSION]) -> Vec3 {
        Vec3 { e: e }
    }
            
    pub fn x(&self) -> f64 { self[0] }
    pub fn y(&self) -> f64 { self[1] }
    pub fn z(&self) -> f64 { self[2] }

    pub fn reverse(&self) ->Vec3 {
        let mut e: [f64; DIMENSION] = [0.0; DIMENSION];
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

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}


// some simple tests
#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn index_access() {
        let v = Vec3::new([1.0, 2.0, 3.0]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn index_modify() {
        let mut v = Vec3::new([0.0, 1.0, 3.0]);
        v[0] += 1.0;
        assert_eq!(v[0], 1.0);

        v[0] = v[2];
        assert_eq!(v[0], 3.0); 
    }

    #[test]
    #[should_panic]
    fn index_out_of_range() {
        let v = Vec3::new([0.0, 0.0, 0.0]);
        assert_eq!(v[3], 0.0);
    }

    #[test]
    fn reverse() {
        let v = Vec3::new([1.0, 2.0, 3.0]);
        let mut rv = v.reverse();
        rv = rv.reverse();
        assert_eq!(v, rv);
    }

    #[test]
    fn square_length_unit() {
        let (e1, e2, e3) = (1.0, 2.0, 3.0);
        let v = Vec3::new([1.0, 2.0, 3.0]);
        let square: f64 = e1 * e1 + e2 * e2 + e3 * e3;
        let sqrt = square.sqrt();
        assert_eq!(square, v.square());
        assert_eq!(sqrt, v.length());
        assert_eq!(v.unit(), v / sqrt);
    }

    #[test]
    fn dot_cross() {
        let (ei0, ei1, ei2) = (2.0, 3.0, 4.0);
        let (ej0, ej1, ej2) = (9.0, 4.0, 5.0);
        let v1 = Vec3::new([ei0, ei1, ei2]);
        let v2 = Vec3::new([ej0, ej1, ej2]);
        let expect_dot = ei0 * ej0 + ei1 * ej1 + ei2 * ej2;
        let expect_cross = Vec3::new([
            ei1 * ej2 - ei2 * ej1,
            ei2 * ej0 - ei0 * ej2,
            ei0 * ej1 - ei1 * ej0
        ]);
        assert_eq!(expect_dot, v1.dot(&v2));
        assert_eq!(expect_cross, v1.cross(&v2));
    }

    #[test]
    fn arithmetic() {
        let (ei0, ei1, ei2) = (2.0, 3.0, 4.0);
        let (ej0, ej1, ej2) = (9.0, 4.0, 5.0);
        let v1 = Vec3::new([ei0, ei1, ei2]);
        let v2 = Vec3::new([ej0, ej1, ej2]);

        assert_eq!(v1 + v2, v2 + v1);
        assert_eq!(v1 - v2, (v2 - v1).reverse());
        assert_eq!(v1 * v2, v1 * v2);

        let k = 4.0;
        assert_eq!(v1 + k, Vec3::new([ei0 + k, ei1 + k, ei2 + k]));
        assert_eq!(v1 - k, Vec3::new([ei0 - k, ei1 - k, ei2 - k]));
        assert_eq!(v1 * k, Vec3::new([ei0 * k, ei1 * k, ei2 * k]));
        assert_eq!(v1 / k, Vec3::new([ei0 / k, ei1 / k, ei2 / k]));
    }
}