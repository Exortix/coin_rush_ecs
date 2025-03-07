use specs::*;

use crate::components::{Position, Velocity};

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x += velocity.dx * 5.0; // Scale movement speed
            position.y += velocity.dy * 5.0;
        }
    }
}
