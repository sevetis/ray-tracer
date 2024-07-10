use super::vec3::{Point, Vec3};
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

    pub fn ray_intersect(&self, ray: &Ray) -> Option<Point> {
        let oc = self.center - *ray.org();
        let a = ray.direct().square();
        let h = ray.direct().dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let delta = h * h - a * c;
        if delta < 0.0 {
            None
        } else {
            let x1 = (h - delta.sqrt()) / a;
            Some(ray.range(x1))
        }
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::{Sphere, Vec3, Point, Ray};

    #[test]
    fn intersect() {
        let ball = Sphere::new(
            Point::new([0.0, 0.0, 0.0]),
            1.0
        );
        let r1 = Ray::new(
            Point::new([1.0, 0.0, 0.0]),
            Vec3::new([0.0, 0.0, 1.0])
        );
        let r2 = Ray::new(
            Point::new([0.0, 0.0, 0.0]),
            Vec3::new([0.0, 0.0, 1.0])
        );
        let r3 = Ray::new(
            Point::new([2.0, 0.0, 0.0]),
            Vec3::new([0.0, 0.0, 2.0])
        );

        assert_eq!(ball.ray_intersect(&r1), Some(Point::new([1.0, 0.0, 0.0])));
        assert_eq!(ball.ray_intersect(&r2), Some(Point::new([0.0, 0.0, -1.0])));
        assert_eq!(ball.ray_intersect(&r3), None);
    }
}
