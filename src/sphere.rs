use super::vec3::{Point};
use super::ray::{Ray};

pub struct Sphere {
    center: Point,
    radius: f64
}

impl Sphere {
    pub const fn new(p: Point, r: f64) -> Sphere {
        Sphere {
            center: p,
            radius: r
        }
    }

    fn ray_distance(&self, ray: &Ray) -> f64 {
        let a = ray.direct();
        let b = self.center - *ray.org();
        let a_len = a.length();
        let b_len = b.length();
        let cos_ab = a.dot(&b) / (a_len * b_len);
        let sin_ab = (1.0 - cos_ab * cos_ab).sqrt();
        sin_ab * b_len
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        if self.center == *ray.org() {
            return true;
        }
        self.ray_distance(ray) <= self.radius
    }
}