use crate::ray::{Ray, HitRecord, Hittable};
use crate::vec3::{Point};
use std::sync::{Arc};

pub const INF: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
pub const ORIGIN: Point = Point::new([0.0, 0.0, 0.0]);

pub struct World {
    objects: Vec<Arc<dyn Hittable>>
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new()
        }
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Arc::new(object));
    }
    
}

impl Hittable for World {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut result = None;

        for obj in self.objects.iter() {
            if let Some(rec) = obj.intersect(ray, t_min, closest) {
                closest = rec.t();
                result = Some(rec);
            }
        }

        result
    }
}