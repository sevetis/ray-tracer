use crate::vec3::{Point, Color, Vec3};
use crate::ray::{Ray, RayHit};
use crate::world::{ORIGIN, INF};
use std::fs::File;
use std::io::prelude::*;


const ASPECT_RATIO: f64 = 16.0 / 9.0;
const FOCAL_LEN: f64 = 1.0;
const WIDTH: f64 = 400.0;
const APPROACH: f64 = 255.999;

const WHITE: Color = Color::new([1.0, 1.0, 1.0]);
const BLUE: Color = Color::new([0.0, 0.0, 1.0]);

fn ray_color(r: &Ray, world: &dyn RayHit) -> Color {
    match world.intersect(r, 0.0, INF) {
        Some(rec) => 0.5 * (WHITE + *rec.normal()),
        None => {
            let alpha = (r.direct().unit().y() + 1.0) / 2.0;
            alpha * WHITE + (1.0 - alpha) * BLUE
        }
    }
}

fn write_color(mut file: &File, c: &Color) {
    let r_byte = (c.x() * APPROACH) as i32;
    let g_byte = (c.y() * APPROACH) as i32;
    let b_byte = (c.z() * APPROACH) as i32;
    let color = format!("{} {} {}\n", r_byte, g_byte, b_byte);
    let _ = file.write_all(color.as_bytes());
}


pub struct Camera {
    eye: Point,
    width: f64,
    height: f64,
    pixel_start: Point,
    delta_u: Vec3,
    delta_v: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        let width = WIDTH;
        let height = (width / ASPECT_RATIO).max(1.0).floor();

        let viewport_height = 2.0;
        let viewport_width = viewport_height * ASPECT_RATIO;
        let viewport_u = Vec3::new([viewport_width, 0.0, 0.0]);
        let viewport_v = Vec3::new([0.0, -viewport_height, 0.0]);

        let delta_u = viewport_u / width;
        let delta_v = viewport_v / height;
        let viewport_upper_left = ORIGIN + Vec3::new([0.0, 0.0, -FOCAL_LEN]) - viewport_u / 2.0 - viewport_v / 2.0;
        let start = viewport_upper_left + (delta_u + delta_v) / 2.0;
        
        Camera {
            eye: ORIGIN,
            width: width,
            height: height,
            pixel_start: start,
            delta_u: delta_u,
            delta_v: delta_v
        }
    }

    pub fn render<T: RayHit + 'static>(&self, world: &T) {
        let mut photo = match File::create("out.ppm") {
            Err(e) => panic!("Could not create photo: {}", e),
            Ok(file) => file
        };
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let _ = photo.write_all(header.as_bytes());

        for i in 0..self.height as i64 {
            for j in 0..self.width as i64 {
                let pixel_center = self.pixel_start + (i as f64) * self.delta_v + (j as f64) * self.delta_u;
                let ray = Ray::new(self.eye, pixel_center - self.eye);
                write_color(&photo, &ray_color(&ray, world));
            }
        }
    }
}