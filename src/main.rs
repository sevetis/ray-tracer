use lib::{Material, Camera, World, Sphere, Point, Color, ORIGIN};
use std::sync::Arc;
use rand::Rng;

fn main() {
    let mut world = World::new();

    let material_ground = Material::Lambertian(Color::new([0.5, 0.5, 0.5]));
    let earth = Sphere::new(Point::new([0.0, -1000.0, 0.0]), 1000.0, material_ground);
    world.add(Arc::new(earth));

    let mut rng = rand::thread_rng();
    for i in -8..8 {
        for j in -8..8 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let radius = rng.gen_range(0.1..0.2);
            let center = Point::new([
                i as f64 + 0.9 * rng.gen_range(0.0..1.0), 
                radius,
                j as f64 + 0.9 * rng.gen_range(0.0..1.0)
            ]);

            if (center - Point::new([4.0, radius, 0.0])).length() > 0.9 {
                let sphere_mat = if choose_mat < 0.3 {
                    let albedo = Color::random(0.0, 0.6);
                    Material::Lambertian(albedo)
                } else if choose_mat < 0.9 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.4);
                    Material::Metal(albedo, fuzz)
                } else {
                    Material::Dielectric(rng.gen_range(0.5..2.0))
                };

                world.add(Arc::new(Sphere::new(center, radius, sphere_mat)));
            }
        }
    }

    let material_big_ball_1 = Material::Lambertian(Color::new([0.8, 0.65, 0.3]));
    let big_ball_1 = Sphere::new(Point::new([-150.0, 69.0, -30.0]), 80.0, material_big_ball_1);
    world.add(Arc::new(big_ball_1));
    
    let material_big_ball_2 = Material::Dielectric(1.5);
    let big_ball_2 = Sphere::new(Point::new([-4.0, 1.0, 0.0]), 1.0, material_big_ball_2);
    world.add(Arc::new(big_ball_2));

    let material_big_ball_3 = Material::Metal(Color::new([0.5, 0.6, 0.7]), 0.0);
    let big_ball_3 = Sphere::new(Point::new([4.0, 1.0, 0.0]), 1.0, material_big_ball_3);
    world.add(Arc::new(big_ball_3));

    let c = Camera::new(Point::new([13.0, 2.0, 3.0]), ORIGIN);
    c.render(Arc::new(world));
}

