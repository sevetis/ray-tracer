use lib::{Camera, World, Sphere, Point};

fn main() {

    let mut world = World::new();
    let ball = Sphere::new(Point::new([0.0, 0.0, -1.0]), 0.5);
    world.add(ball);

    let c = Camera::new();
    c.render(&world)
}

