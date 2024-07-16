use crate::ray::{Ray, Hittable};
use crate::vec3::{Point, Vec3};
use crate::world::{ORIGIN, PI};
use crate::color::*;
use std::fs::File;
use std::io::Write;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const V_FOV: f64 = 90.0;    // vertical field of view
const WIDTH: f64 = 1000.0;

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
    pub fn new(look_from: Point, look_at: Point) -> Camera {
        let width = WIDTH;
        let height = (width / ASPECT_RATIO).max(1.0).floor();

        let focal_len = (look_from - look_at).length();
        
        let theta = V_FOV * PI / 180.0; // degree to radian
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_len;
        let viewport_width = viewport_height * ASPECT_RATIO;
        
        let vup = Vec3::new([0.0, 1.0, 0.0]);
        let w = (look_from - look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);
        
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * v.reverse();

        let delta_u = viewport_u / width;
        let delta_v = viewport_v / height;
        let viewport_upper_left = ORIGIN - focal_len * w - viewport_u / 2.0 - viewport_v / 2.0;
        let start = viewport_upper_left + (delta_u + delta_v) / 2.0;
        
        Camera {
            eye: look_from,
            width: width,
            height: height,
            pixel_start: start,
            delta_u: delta_u,
            delta_v: delta_v,
            sample_num: 10,
            reflect_depth: 50,
        }
    }

    pub fn render<T: Hittable + 'static>(&self, environment: &T) {
        let mut photo = match File::create("out.ppm") {
            Err(e) => panic!("Could not create photo: {}", e),
            Ok(file) => file,
        };
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let _ = photo.write_all(header.as_bytes());

        let height = self.height as i64;
        let width = self.width as i64;

        for i in 0..height {
            let y = i as f64;
            print!("\rProgress: {:.0}%", y * 100.0 / self.height);
            std::io::stdout().flush().unwrap();
            
            for j in 0..width {
                let x = j as f64;
                let mut color = BLACK;
                for _ in 0..self.sample_num {
                    let offset = Vec3::random(-0.5, 0.5);
                    let mut sample_pixel = self.pixel_start;
                    sample_pixel = sample_pixel + (y + offset.y()) * self.delta_v;
                    sample_pixel = sample_pixel + (x + offset.x()) * self.delta_u;
                    let ray = Ray::new(self.eye, sample_pixel - self.eye);
                    color = color + ray_color(&ray, environment, self.reflect_depth);
                }
                
                let samples_average_color = color / self.sample_num as f64;
                write_color(&photo, &samples_average_color);
            }
        }
        println!("\nCompleted!");
    }

}