use crate::vec3::{Point, Vec3};

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
        let reflect = Vec3::random_in_unit_sphere().unit();
        if reflect.dot(normal) > 0.0 {
            reflect
        } else {
            reflect.reverse()
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct HitRecord {
    t: f64,
    pos: Point,
    normal: Vec3,
    front_face: bool
}

impl HitRecord {
    pub fn new(t: f64, p: Point, n: Vec3, front: bool) -> HitRecord {
        HitRecord {
            t: t,
            pos: p,
            normal: n,
            front_face: front
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
}

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[cfg(test)]
mod tests {
    use super::{Ray, Point, Vec3};

    #[test]
    fn ray() {
        let o = Point::new([0.0, 0.0, 0.0]); 
        let d = Vec3::new([1.0, 0.0, 0.0]);
        let r = Ray::new(o, d);
        assert_eq!(o, *r.org());
        assert_eq!(d, *r.direct());
        assert_eq!(o + d.unit() * 3.0, r.range(3.0));
    }

}
