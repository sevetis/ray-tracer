use crate::vec3::{Point, Vec3};
use crate::material::{Material};

pub struct Ray {
    origin: Point,
    direction: Vec3
}

impl Ray {
    pub fn new(org: Point, direct: Vec3) -> Ray {
        Ray {
            origin: org, 
            direction: direct
        }
    }

    pub fn org(&self) -> &Point {
        &self.origin
    }

    pub fn direct(&self) -> &Vec3 {
        &self.direction
    }

    pub fn range(&self, pos: f64) -> Point {
        self.origin + self.direction * pos
    }

    pub fn diffuse(normal: &Vec3) -> Vec3 {
        let reflect = Vec3::random_unit_vec();
        if reflect.dot(normal) > 0.0 {
            reflect
        } else {
            reflect.reverse()
        }
    }
}


#[derive(PartialEq)]
pub struct HitRecord {
    t: f64,
    pos: Point,
    normal: Vec3,
    front_face: bool,
    mat: Material,
}

impl HitRecord {
    pub fn new(t: f64, p: Point, n: Vec3, front: bool, m: Material) -> HitRecord {
        HitRecord {
            t: t,
            pos: p,
            normal: n,
            front_face: front,
            mat: m,
        } 
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn pos(&self) -> &Point {
        &self.pos
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn mat(&self) -> &Material {
        &self.mat
    }
}

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

