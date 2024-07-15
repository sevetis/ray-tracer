use crate::ray::{Ray, HitRecord, Hittable};
use crate::material::{Material};
use crate::vec3::{Point};

pub struct Sphere {
    center: Point,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub const fn new(p: Point, r: f64, m: Material) -> Sphere {
        Sphere {
            center: p,
            radius: r,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = self.center - *ray.org();
        let a = ray.direct().square();
        let h = ray.direct().dot(&oc);
        let c = oc.square() - self.radius * self.radius;
        let delta = h * h - a * c;

        if delta < 0.0 {
            return None;
        }

        let delta_sqrt = delta.sqrt();
        let x1 = (h - delta_sqrt) / a;
        let x2 = (h + delta_sqrt) / a;

        let root: f64;
        if t_min <= x1 && x1 <= t_max {
            root = x1;
        } else if t_min <= x2 && x2 <= t_max {
            root = x2;
        } else {
            return None;
        }

        let position = ray.range(root);
        let mut normal = (position - self.center) / self.radius;
        let front_face = ray.direct().dot(&normal) < 0.0;
        if !front_face { normal = normal.reverse(); }

        Some(HitRecord::new(
            root,
            position,
            normal,
            front_face,
            self.mat,
        ))
    }
}

impl Hittable for &Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (*self).intersect(ray, t_min, t_max)
    }
}

