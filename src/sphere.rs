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
        if self.center == *ray.org() {
            return 0.0;
        }

        let a = ray.direct();
        let b = self.center - *ray.org();
        let a_len = a.length();
        let b_len = b.length();
        let cos_ab = a.dot(&b) / (a_len * b_len);
        let sin_ab = (1.0 - cos_ab * cos_ab).sqrt();
        sin_ab * b_len
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        self.ray_distance(ray) <= self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::Sphere;

    #[test]
    fn intersect() {
        let ball = Sphere::new(
            Point::new([0.0, 0.0, 0.0]),
            1.0
        );
        let r1 = Ray::new(
            Point::new([1.0, 0.0, 0.0]),
            Vec3::new([0.0, 0.0, 1.0])
        )
        let r2 = Ray::new(
            Point::new([0.0, 0.0, 0.0]),
            Vec3::new([0.0, 0.0, 1.0])
        )
        let r3 = Ray::new(
            Point::new([2.0, 0.0, 0.0]),
            Vec3::new([0.0, 0.0, 2.0])
        )

        assert_eq!(ball.intersect(r1), true);
        assert_eq!(ball.intersect(r2), true);
        assert_eq!(ball.intersect(r3), false);
    }
}