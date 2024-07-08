use crate::vec3::{Point, Vec3};

pub struct Ray {
    origin: Point,
    direction: Vec3
}

impl Ray {
    pub fn new(org: Point, direct: Vec3) -> Ray {
        Ray {
            origin: org, 
            direction: direct.unit()
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