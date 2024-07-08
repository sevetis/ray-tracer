mod vec3;
use vec3::{Color};

const APPROACH: f64 = 255.999;

fn write_color(c: &Color) {
    let r_byte = (c.x() * APPROACH) as i32;
    let g_byte = (c.y() * APPROACH) as i32;
    let b_byte = (c.z() * APPROACH) as i32;
    println!("{} {} {}", r_byte, g_byte, b_byte);
}

fn main() {
    let width: f64 = 256.0;
    let height: f64 = 256.0;
    println!("P3");
    println!("{} {}", width, height);
    println!("255");


    for i in 0..height as i32 {
        for j in 0..width as i32 {
            let c = Color::new([
                0.0,
                i as f64 / (height - 1.0),
                j as f64 / (width - 1.0)
            ]);
            write_color(&c);
        }
    }
}

