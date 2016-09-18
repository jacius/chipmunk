extern crate chipmunk;

use chipmunk::{Body, BodyHandle, Shape, ShapeHandle, Space};

fn main() {
    // The space contains everything in the simulation.
    let mut space = Space::new();
    space.set_gravity((0.0, -100.0));

    // Set up a floor for our ball to bounce off of.
    let mut floor_body = BodyHandle::new_static();
    let mut floor_shape = {
        let mut s = Shape::new_segment(&mut floor_body, (-10.0, 0.0), (10.0, 0.0), 0.0);
        s.set_elasticity(0.6);
        ShapeHandle::from(s)
    };
    space.add_body(&mut floor_body);
    space.add_shape(&mut floor_shape);

    // Add a bouncing ball to the scene.
    let mut ball_body = {
        let mut b = Body::new(0.0, 0.0);
        b.set_position((0.0, 20.0));
        BodyHandle::from(b)
    };

    let mut ball_shape = {
        let mut s = Shape::new_circle(&mut ball_body, 1.0, (0.0, 0.0));
        s.set_mass(10.0);
        s.set_elasticity(0.9);
        ShapeHandle::from(s)
    };

    space.add_body(&mut ball_body);
    space.add_shape(&mut ball_shape);

    // Run the simulation!
    for _ in 0..40 {
        space.step(1.0/30.0);
        let pos = ball_body.borrow().position();
        let y = (4.0 * pos.y).round().max(0.0);
        let s: String = ::std::iter::repeat(' ').take(y as usize).collect();
        println!("{}o", s);
    }
}
