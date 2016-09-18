extern crate chipmunk;
extern crate lux;
extern crate rand;

use chipmunk::{Body, BodyHandle, Shape, ShapeHandle, Space};
use lux::prelude::*;
use lux::game::Game;
use rand::Rng;

// Width and height of the window (and room)
static WIDTH: u32 = 800;
static HEIGHT: u32 = 500;


struct MyGame {
    pub space: Space,
    pub balls: Vec<Ball>,
}

impl Game for MyGame {
    fn update(&mut self, dt: f32, _window: &mut Window, _events: &mut EventIterator) -> LuxResult<()> {
        self.space.step(dt as f64);
        Ok(())
    }

    fn render(&mut self, _lag: f32, _window: &mut Window, frame: &mut Frame) -> LuxResult<()> {
        for ref ball in &self.balls {
            ball.render(frame);
        }
        Ok(())
    }

    fn show_fps(&self, window: &Window) -> bool {
        window.is_key_pressed(' ')
    }
}


struct Ball {
    pub body: BodyHandle,
    pub shape: ShapeHandle,
    pub color: [f32; 4],
}

impl Ball {
    fn render(&self, frame: &mut Frame) {
        let pos = self.body.borrow().position();
        let rot = self.body.borrow().rotation();

        let x = pos.x as f32;
        let y = (HEIGHT as f64 - pos.y) as f32;

        let radius = {
            match *self.shape.borrow() {
                Shape::Circle(ref circle) => circle.radius() as f32,
                _ => 0.0,
            }
        };

        // Draw colored circle
        frame.circle(x - radius, y - radius, 2.0 * radius)
            .color(self.color)
            .fill();

        // Draw black line from the center outward
        let x2 = x + radius * rot.x as f32;
        let y2 = y + radius * -rot.y as f32;
        frame.draw_line(x, y, x2, y2, 2.0);
    }
}


fn main() {
    // The space contains everything in the simulation.
    let mut space = Space::new();
    space.set_gravity((0.0, -500.0));

    // Set up a room with walls and a floor.
    let width = WIDTH as f64;
    let height = HEIGHT as f64;
    let walls = vec![
        ((0.0, 0.0),  (0.0, height)), // left
        ((0.0, 0.0),  (width, 0.0)), // bottom
        ((width, 0.0), (width, height)), // right
        ((0.0, height), (width, height)), // top
    ];

    let mut room = BodyHandle::new_static();
    for wall in walls {
        let mut shape = Shape::new_segment(&mut room, wall.0, wall.1, 0.0);
        shape.set_elasticity(0.8);
        shape.set_friction(0.75);
        space.add_shape(&mut ShapeHandle::from(shape));
    }
    space.add_body(&mut room);

    // Create many random bouncing balls and add them to the scene.
    let mut rng = rand::thread_rng();
    let balls = (0..50).map(|_| {
        let radius = rng.gen_range(10.0, 40.0);
        let color = [rng.gen_range(0.0, 0.7),
                     rng.gen_range(0.0, 0.7),
                     rng.gen_range(0.0, 0.7),
                     1.0];
        let pos = (rng.gen_range(radius, WIDTH as f64 - radius),
                   rng.gen_range(radius, HEIGHT as f64 - radius));
        let vel = (rng.gen_range(-200.0, 200.0),
                   rng.gen_range(-200.0, 200.0));
        let avel = rng.gen_range(-8.0, 8.0);

        let mut body = {
            let mut b = Body::new(0.0, 0.0);
            b.set_position(pos);
            b.set_velocity(vel);
            b.set_angular_velocity_rad(avel);
            BodyHandle::from(b)
        };

        let mut shape = {
            let mut s = Shape::new_circle(&mut body, radius, (0.0, 0.0));
            s.set_density(1.0);
            s.set_elasticity(0.85);
            s.set_friction(0.70);
            ShapeHandle::from(s)
        };

        space.add_body(&mut body);
        space.add_shape(&mut shape);

        Ball {
            body: body,
            shape: shape,
            color: color,
        }
    }).collect();

    let game = MyGame {
        space: space,
        balls: balls,
    };

    game.run_until_end().unwrap();
}
