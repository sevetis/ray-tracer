
fn main() {
    let width: f64 = 256.0;
    let height: f64 = 256.0;
    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for i in 0..height as i32 {
        for j in 0..width as i32 {
            let r: f64 = 0.0; 
            let g: f64 = j as f64 / (height - 1.0);
            let b: f64 = i as f64 / (width - 1.0);

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

