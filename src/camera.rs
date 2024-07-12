use crate::vec3::{Point, Color, Vec3};
use crate::ray::{Ray, Hittable};
use crate::world::{ORIGIN, INF};
use std::fs::File;
use std::io::prelude::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const FOCAL_LEN: f64 = 1.0;
const WIDTH: f64 = 400.0;

const RGB_MAX: f64 = 255.999;
const WHITE: Color = Color::new([1.0, 1.0, 1.0]);
const BLACK: Color = Color::new([0.0, 0.0, 0.0]);
const SKY_BLUE: Color = Color::new([0.5, 0.7, 1.0]);

fn ray_color<T: Hittable + 'static>(r: &Ray, environment: &T, depth: u8) -> Color {
    if depth <= 0 { return BLACK; }
    match environment.intersect(r, 0.0, INF) {
        Some(rec) => {
            let reflect = Ray::diffuse(rec.normal());
            0.5 * ray_color(&Ray::new(*rec.pos(), reflect), environment, depth - 1)
        },
        None => {
            let alpha = (r.direct().unit().y() + 1.0) / 2.0;
            (1.0 - alpha) * WHITE + alpha * SKY_BLUE
        }
    }
}

fn write_color(mut file: &File, c: &Color) {
    let r_byte = (c.x() * RGB_MAX) as i32;
    let g_byte = (c.y() * RGB_MAX) as i32;
    let b_byte = (c.z() * RGB_MAX) as i32;
    let color = format!("{} {} {}\n", r_byte, g_byte, b_byte);
    let _ = file.write_all(color.as_bytes());
}


pub struct Camera {
    eye: Point,
    width: f64,
    height: f64,
    pixel_start: Point,
    delta_u: Vec3,
    delta_v: Vec3,
    sample_num: u16,
    reflect_depth: u8,
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
            delta_v: delta_v,
            sample_num: 10,
            reflect_depth: 50
        }
    }

    pub fn render<T: Hittable + 'static>(&self, environment: &T) {
        let mut photo = match File::create("out.ppm") {
            Err(e) => panic!("Could not create photo: {}", e),
            Ok(file) => file
        };
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let _ = photo.write_all(header.as_bytes());

        for i in 0..self.height as i64 {
            println!("Remaining: {}", self.height as i64 - i);
            for j in 0..self.width as i64 {
                let mut color = BLACK;
                for _ in 0..self.sample_num {
                    let ray = self.ray_sample(i as f64, j as f64);
                    color = color + ray_color(&ray, environment, self.reflect_depth);
                }
                write_color(&photo, &(color / self.sample_num as f64));
            }
        }
    }

    fn ray_sample(&self, i: f64, j: f64) -> Ray {
        let offset = Vec3::random(-0.5, 0.5);
        let sample_pixel = self.pixel_start + (i + offset.y()) * self.delta_v + (j + offset.x()) * self.delta_u;
        Ray::new(self.eye, sample_pixel - self.eye)
    }
}