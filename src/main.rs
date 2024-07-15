use lib::{Material, Camera, World, Sphere, Point, Color};

fn main() {
    let mut world = World::new();
    let material_ground = Material::Lambertian(Color::new([0.8, 0.8, 0.0]));
    let material_center_ball = Material::Lambertian(Color::new([0.1, 0.2, 0.5]));
    // let material_left_ball = Material::Metal(Color::new([0.8, 0.8, 0.8]), 0.0);
    let material_left_ball = Material::Dielectric(1.50);
    let material_right_ball = Material::Metal(Color::new([0.8, 0.6, 0.2]), 0.3);

    let earth = Sphere::new(Point::new([0.0, -100.5, -1.0]), 100.0, material_ground);
    let center_ball = Sphere::new(Point::new([0.0, 0.0, -1.2]), 0.5, material_center_ball);
    let left_ball = Sphere::new(Point::new([-1.0, 0.0, -1.0]), 0.5, material_left_ball);
    let right_ball = Sphere::new(Point::new([1.0, 0.0, -1.0]), 0.5, material_right_ball);

    world.add(earth);
    world.add(center_ball);
    world.add(left_ball);
    world.add(right_ball);

    let c = Camera::new();
    c.render(&world)
}

