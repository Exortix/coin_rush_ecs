mod components;
mod entities;
mod resources;
mod systems;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use specs::prelude::*;
use std::time::Duration;

fn main() -> Result<(), String> {
    // SDL2 Initialization
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust ECS Game", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    // ECS World Setup
    let mut world = World::new();
    components::register_components(&mut world);
    world.insert(resources::InputResource::default());

    // Create Entities
    entities::create_player(&mut world);
    entities::create_initial_coin(&mut world);
    entities::create_initial_enemy(&mut world);

    // Initialize Systems
    let mut render_system = systems::RenderSystem {
        canvas: &mut canvas,
    };
    let mut player_input_system = systems::PlayerInputSystem;
    let mut physics_system = systems::PhysicsSystem;
    let mut spawn_system = systems::SpawnSystem::new(800, 600);

    // Game Loop
    let mut running = true;
    while running {
        let keyboard_state = event_pump.keyboard_state();
        let keys: Vec<Scancode> = keyboard_state.pressed_scancodes().collect();
        world.write_resource::<resources::InputResource>().keys =
            keys.iter().map(|k| Some(*k)).collect();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false,
                _ => {}
            }
        }

        // Update Systems
        player_input_system.run_now(&world);
        physics_system.run_now(&world);
        spawn_system.run_now(&world);
        render_system.run_now(&world);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
