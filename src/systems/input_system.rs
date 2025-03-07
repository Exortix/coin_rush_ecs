use sdl2::keyboard::Scancode;
use specs::prelude::*;

use crate::{
    components::{PlayerControlled, Velocity},
    resources::InputResource,
};

pub struct PlayerInputSystem;

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        ReadExpect<'a, InputResource>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, PlayerControlled>,
    );

    fn run(&mut self, (input, mut velocities, players): Self::SystemData) {
        for (velocity, _) in (&mut velocities, &players).join() {
            velocity.dx = 0.0;
            velocity.dy = 0.0;
            for key in &input.keys {
                if let Some(scancode) = key {
                    match scancode {
                        Scancode::W => velocity.dy -= 1.0,
                        Scancode::S => velocity.dy += 1.0,
                        Scancode::A => velocity.dx -= 1.0,
                        Scancode::D => velocity.dx += 1.0,
                        _ => {}
                    }
                }
            }
        }
    }
}
