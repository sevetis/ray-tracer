mod vec3;
mod ray;

use vec3::{Point, Color, Vec3};
use ray::{Ray};


const ASPECT_RATIO: f64 = 16.0 / 9.0;
const FOCAL_LEN: f64 = 1.0;

const APPROACH: f64 = 255.999;
fn write_color(c: &Color) {
    let r_byte = (c.x() * APPROACH) as i32;
    let g_byte = (c.y() * APPROACH) as i32;
    let b_byte = (c.z() * APPROACH) as i32;
    println!("{} {} {}", r_byte, g_byte, b_byte);
}

// gradient
const START_COLOR: Color = Color::new([1.0, 1.0, 1.0]);
const END_COLOR: Color = Color::new([0.0, 0.0, 1.0]);
fn ray_color(r: &Ray) -> Color {
    let alpha = (r.direct().unit().y() + 1.0) / 2.0;
    alpha * START_COLOR + (1.0 - alpha) * END_COLOR
}

fn main() {
    let width = 400.0;
    let height = (width / ASPECT_RATIO).max(1.0).floor();

    // write meta data;
    println!("P3");
    println!("{} {}", width as i32, height as i32);
    println!("255");

    let viewport_height = 2.0;
    let viewport_width = viewport_height * height / height;

    let eye = Point::new([0.0, 0.0, -FOCAL_LEN]);
    let viewport_u = Vec3::new([viewport_width, 0.0, 0.0]);
    let viewport_v = Vec3::new([0.0, -viewport_height, 0.0]);
    let delta_u = viewport_u / width;
    let delta_v = viewport_v / height;

    let viewport_upper_left = eye + Vec3::new([0.0, 0.0, FOCAL_LEN]) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_start = viewport_upper_left + (delta_u + delta_v) * 0.5;
    
    for i in 0..height as i32 {
        for j in 0..width as i32 {
            let pixel_center = pixel_start + (i as f64 * delta_v) + (j as f64 * delta_u);
            let r = Ray::new(eye, pixel_center - eye);
            write_color(&ray_color(&r));
        }
    }
}

