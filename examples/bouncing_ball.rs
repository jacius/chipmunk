extern crate chipmunk;

use chipmunk::{BodyHandle, ShapeHandle, Space};

fn main() {
    // The space contains everything in the simulation.
    let mut space = Space::new();
    space.set_gravity((0.0, -100.0));

    // Set up a floor for our ball to bounce off of.
    let mut floor_body = BodyHandle::new_static();
    let mut floor_shape = ShapeHandle::new_segment(
        &mut floor_body, (-10.0, 0.0), (10.0, 0.0), 0.0);

    floor_shape.write().set_elasticity(0.6);
    space.add_body(&mut floor_body);
    space.add_shape(&mut floor_shape);

    // Add a bouncing ball to the scene.
    let mut ball_body = BodyHandle::new(0.0, 0.0);
    let mut ball_shape = ShapeHandle::new_circle(&mut ball_body, 1.0, (0.0, 0.0));

    ball_body.write().set_position((0.0, 20.0));
    ball_shape.write().set_mass(10.0);
    ball_shape.write().set_elasticity(0.9);

    space.add_body(&mut ball_body);
    space.add_shape(&mut ball_shape);

    // Run the simulation!
    for _ in 0..40 {
        space.step(1.0/30.0);
        let pos = ball_body.read().position();
        let y = (4.0 * pos.y).round().max(0.0);
        let s: String = ::std::iter::repeat(' ').take(y as usize).collect();
        println!("{}o", s);
    }
}
