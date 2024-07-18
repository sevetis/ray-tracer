use crate::ray::{Ray, Hittable};
use crate::vec3::{Point, Vec3};
use crate::color::*;
use std::fs::File;
use std::io::Write;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const V_FOV: f64 = 20.0;    // vertical field of view
const WIDTH: f64 = 1200.0;

pub struct Camera {
    eye: Point,
    width: f64,
    height: f64,
    pixel_start: Point,
    delta_u: Vec3,
    delta_v: Vec3,
    sample_num: u16,
    reflect_depth: u8,
    defocus_angle: f64,
    disk_u: Vec3,
    disk_v: Vec3,
}

impl Camera {
    pub fn new(look_from: Point, look_at: Point) -> Camera {
        let width = WIDTH;
        let height = (width / ASPECT_RATIO).max(1.0).floor();

        let focus_dist = 10.0;
        let defocus_angle = 0.6;
        let theta = V_FOV.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * ASPECT_RATIO;
        
        let vup = Vec3::new([0.0, 1.0, 0.0]);
        let w = (look_from - look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);
        
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * v.reverse();

        let delta_u = viewport_u / width;
        let delta_v = viewport_v / height;
        let viewport_upper_left = look_from - focus_dist * w - viewport_u / 2.0 - viewport_v / 2.0;
        let start = viewport_upper_left + (delta_u + delta_v) / 2.0;
        
        let defocus_radius = focus_dist * f64::from(defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            eye: look_from,
            width: width,
            height: height,
            pixel_start: start,
            delta_u: delta_u,
            delta_v: delta_v,
            sample_num: 500,
            reflect_depth: 50,
            defocus_angle: defocus_angle,
            disk_u: defocus_disk_u,
            disk_v: defocus_disk_v,
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
                    let sample_pixel = self.pixel_start
                        + (y + offset.y()) * self.delta_v
                        + (x + offset.x()) * self.delta_u;
                    
                    let ray_org: Point;
                    if self.defocus_angle <= 0.0 {
                        ray_org = self.eye;
                    } else {
                        ray_org = self.defocus_sample();
                    }
                    let ray = Ray::new(ray_org, sample_pixel - ray_org);
                    color = color + ray_color(&ray, environment, self.reflect_depth);
                }
                
                let samples_average_color = color / self.sample_num as f64;
                write_color(&photo, &samples_average_color);
            }
        }
        println!("\nCompleted!");
    }

    fn defocus_sample(&self) -> Point {
        let p = Vec3::random_in_unit_disk();
        self.eye + p.x() * self.disk_u + p.y() * self.disk_v
    }
}