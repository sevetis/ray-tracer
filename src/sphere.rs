use super::vec3::{Point};
use super::ray::{Ray, HitRecord, RayHit};

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

//    pub fn center(&self) -> &Point {
//        &self.center
//    }

//    pub fn radius(&self) -> f64 {
//        self.radius
//    }
}

impl RayHit for Sphere {

    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = self.center - *ray.org();
        let a = ray.direct().square();
        let h = ray.direct().dot(&oc);
        let c = oc.square() - self.radius * self.radius;
        let delta = h * h - a * c;

        if delta < 0.0 {
            return None;
        } else {
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
            let rec = HitRecord::new(
                root,
                position,
                (position - self.center) / self.radius
            );
            Some(rec)
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;
    use super::{Sphere, Point, Ray, RayHit};

    #[test]
    fn intersect() {
        let ball = Sphere::new(
            Point::new([0.0, 0.0, 0.0]),
            1.0
        );
        let r1 = Ray::new(
            Point::new([2.0, 0.0, -1.0]),
            Vec3::new([-2.0, 0.0, 2.0])
        );
        let r2 = Ray::new(
            Point::new([0.0, 0.0, 0.0]),
            Vec3::new([0.0, 0.0, 1.0])
        );
        let r3 = Ray::new(
            Point::new([2.0, 0.0, 0.0]),
            Vec3::new([0.0, 0.0, 2.0])
        );

        let mut result;

        result = ball.intersect(&r1, 0.0, 100.0);
        assert_ne!(result, None);
        assert_eq!(result.unwrap().pos(), &Point::new([1.0, 0.0, 0.0]));

        result = ball.intersect(&r2, 0.0, 100.0);
        assert_ne!(result, None);
        assert_eq!(result.unwrap().pos(), &Point::new([0.0, 0.0, 1.0]));

        result = ball.intersect(&r3, 0.0, 100.0);
        assert_eq!(result, None);
    }

}


