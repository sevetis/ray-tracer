use crate::ray::{Ray, Hittable};
use crate::material::{scatter};
use crate::vec3::{Vec3};
use crate::world::{INF};
use std::fs::File;
use std::io::{Write, BufWriter};

pub type Color = Vec3;

const RGB_MAX: f64 = 255.999;
pub const WHITE: Color = Color::new([1.0, 1.0, 1.0]);
pub const BLACK: Color = Color::new([0.0, 0.0, 0.0]);
const SKY_BLUE: Color = Color::new([0.5, 0.7, 1.0]);

pub fn ray_color(r: &Ray, environment: &impl Hittable, depth: u8) -> Color {
    if depth <= 0 { return BLACK; }
    match environment.intersect(r, 0.001, INF) {
        Some(rec) => {
            if let Some((scattered, attenuation)) = scatter(rec.mat(), r, &rec) {
                return attenuation * ray_color(&scattered, environment, depth - 1);
            }
            BLACK
        },
        None => {
            let alpha = (r.direct().unit().y() + 1.0) / 2.0;
            (1.0 - alpha) * WHITE + alpha * SKY_BLUE
        }
    }
}

fn linear_to_gamma(val: f64) -> f64 {
    if val > 0.0 { 
        return val.sqrt();
    }
    0.0
}

pub fn write_color(file: &mut BufWriter<File>, c: &Color) {
    let r_byte = (linear_to_gamma(c.x()) * RGB_MAX) as i32;
    let g_byte = (linear_to_gamma(c.y()) * RGB_MAX) as i32;
    let b_byte = (linear_to_gamma(c.z()) * RGB_MAX) as i32; 
    let color = format!("{} {} {}\n", r_byte, g_byte, b_byte);
    let _ = file.write_all(color.as_bytes());
}
