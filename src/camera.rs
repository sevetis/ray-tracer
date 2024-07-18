use crate::ray::{Ray, Hittable};
use crate::vec3::{Point, Vec3};
use crate::color::*;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::sync::{Arc, Mutex};
use std::thread;


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

    pub fn render<T: Hittable + Sync + Send + 'static>(&self, environment: Arc<T>) {
        let now = std::time::Instant::now();
        let mut photo = match File::create("out.ppm") {
            Err(e) => panic!("Could not create photo: {}", e),
            Ok(file) => BufWriter::new(file),
        };
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let _ = photo.write_all(header.as_bytes());

        let height = self.height as i64;
        let width = self.width as i64;

        // pixel buffer
        let pixels = Arc::new(Mutex::new(vec![BLACK; (width * height) as usize]));

        let num_threads = 16;
        let chunk_size = height / num_threads;
        let mut handles = vec![];

        for thread_id in 0..num_threads {
            let environment = Arc::clone(&environment);
            let pixels = Arc::clone(&pixels);
            let eye = self.eye.clone();
            let pixel_start = self.pixel_start.clone();
            let delta_u = self.delta_u.clone();
            let delta_v = self.delta_v.clone();
            let sample_num = self.sample_num;
            let reflect_depth = self.reflect_depth;
            let defocus_angle = self.defocus_angle;
            let disk_u = self.disk_u.clone();
            let disk_v = self.disk_v.clone();
            let environment = Arc::clone(&environment);
            let pixels = Arc::clone(&pixels);

            let handle = thread::spawn(move || {
                let start_row = thread_id * chunk_size;
                let end_row = if thread_id == num_threads - 1 {
                    height
                } else {
                    (thread_id + 1) * chunk_size
                };

                for i in start_row..end_row {
                    let y = i as f64;
                
                    for j in 0..width {
                        let x = j as f64;
                        let mut color = BLACK;

                        for _ in 0..sample_num {
                            let offset = Vec3::random(-0.5, 0.5);
                            let sample_pixel = pixel_start
                                + (y + offset.y()) * delta_v
                                + (x + offset.x()) * delta_u;
                    
                            let ray_org = if defocus_angle <= 0.0 {
                                eye
                            } else {
                                defocus_sample(eye, disk_u, disk_v)
                            };
                            let ray = Ray::new(ray_org, sample_pixel - ray_org);
                            color = color + ray_color(&ray, &*environment, reflect_depth);
                        }
                
                        let samples_average_color = color / sample_num as f64;

                        let mut pixels = pixels.lock().unwrap();
                        pixels[(i * width + j) as usize] = samples_average_color;
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let pixels = pixels.lock().unwrap();
        for color in pixels.iter() {
            write_color(&mut photo, color);
        }

        if cfg!(target_os = "linux") {
            println!("\nConvert ppm to png");
            convert_ppm_to_png();
        }
        println!("Completed!");
        println!("Rendering time: {}s", now.elapsed().as_secs());
    }
    
}
    
fn defocus_sample(eye: Point, disk_u: Vec3, disk_v: Vec3) -> Point {
    let p = Vec3::random_in_unit_disk();
    eye + p.x() * disk_u + p.y() * disk_v
}

fn convert_ppm_to_png() {
    let output = std::process::Command::new("pnmtopng")
        .arg("out.ppm")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Conversion successful!");
        let mut out_file = File::create("out.png")
            .expect("Failed to create output file");
        std::io::copy(&mut output.stdout.as_slice(), &mut out_file)
            .expect("Failed to write output to file");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Conversion failed:\n{}", stderr);
    }
}