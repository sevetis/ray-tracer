use std::sync::{Arc};
use crate::vec3::{Point};
use crate::ray::{Ray, Hittable, HitRecord};
use crate::sphere::{Sphere};

pub const INF: f64 = std::f64::INFINITY;
pub const ORIGIN: Point = Point::new([0.0, 0.0, 0.0]);
pub const EARTH: Sphere = Sphere::new(Point::new([0.0, -100.5, -1.0]), 100.0);

pub struct World {
    objects: Vec<Arc<dyn Hittable>>
}

impl World {

    pub fn new() -> World {
        let mut world = World::empty_world();
        world.add(&EARTH);
        world
    }

    pub fn empty_world() -> World {
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