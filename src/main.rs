use lib::{Material, Camera, World, Sphere, Point, Color, ORIGIN};
use std::sync::Arc;
use rand::Rng;

fn main() {
    let mut world = World::new();

    let material_ground = Material::Lambertian(Color::new([0.5, 0.5, 0.5]));
    let earth = Sphere::new(Point::new([0.0, -1000.0, 0.0]), 1000.0, material_ground);
    world.add(Arc::new(earth));

    let mut rng = rand::thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point::new([i as f64 + 0.9 * rng.gen_range(0.0..1.0), 0.2, j as f64 + 0.9 * rng.gen_range(0.0..1.0)]);

            if (center - Point::new([4.0, 0.2, 0.0])).length() > 0.9 {
                let sphere_mat: Material;

                if choose_mat < 0.8 {
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    sphere_mat = Material::Lambertian(albedo);
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_mat = Material::Metal(albedo, fuzz);
                } else {
                    sphere_mat = Material::Dielectric(1.5);
                }

                world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
            }
        }
    }

    let material_big_ball_1 = Material::Dielectric(1.5);
    let big_ball_1 = Sphere::new(Point::new([0.0, 1.0, 0.0]), 1.0, material_big_ball_1);
    world.add(Arc::new(big_ball_1));

    let material_big_ball_2 = Material::Lambertian(Color::new([0.4, 0.2, 0.1]));
    let big_ball_2 = Sphere::new(Point::new([-4.0, 1.0, 0.0]), 1.0, material_big_ball_2);
    world.add(Arc::new(big_ball_2));

    let material_big_ball_3 = Material::Metal(Color::new([0.7, 0.6, 0.5]), 0.0);
    let big_ball_3 = Sphere::new(Point::new([4.0, 1.0, 0.0]), 1.0, material_big_ball_3);
    world.add(Arc::new(big_ball_3));

    let c = Camera::new(Point::new([13.0, 2.0, 3.0]), ORIGIN);
    c.render(&world)
}

