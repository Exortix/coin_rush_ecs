use std::time::Duration;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;
use specs::prelude::*;
use specs_derive::*;

// --- Components ---

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Health {
    value: u32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Score {
    value: u32,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct PlayerControlled;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Coin;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Enemy;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
struct Obstacle;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct PowerUp {
    power_type: String,
}

// --- Systems ---

struct RenderSystem<'a> {
    canvas: &'a mut Canvas<Window>,
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, PlayerControlled>,
        ReadStorage<'a, Coin>,
        ReadStorage<'a, Enemy>,
    );

    fn run(&mut self, (positions, players, coins, enemies): Self::SystemData) {
        // self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        //black
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Render Players
        for (pos, _) in (&positions, &players).join() {
            self.canvas.set_draw_color(Color::RGB(0, 255, 0));
            self.canvas
                .fill_rect(sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, 20, 20))
                .unwrap();
        }

        // Render Coins
        for (pos, _) in (&positions, &coins).join() {
            self.canvas.set_draw_color(Color::RGB(255, 255, 0));
            self.canvas
                .fill_rect(sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, 10, 10))
                .unwrap();
        }

        // Render Enemies
        for (pos, _) in (&positions, &enemies).join() {
            self.canvas.set_draw_color(Color::RGB(255, 0, 0));
            self.canvas
                .fill_rect(sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, 20, 20))
                .unwrap();
        }

        self.canvas.present();
    }
}

struct PlayerInputSystem;

#[derive(Default)]
pub struct InputResource {
    pub keys: Vec<Option<Scancode>>, // Stores pressed keys
}

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

struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x += velocity.dx * 5.0; // Scale movement speed
            position.y += velocity.dy * 5.0;
        }
    }
}

struct SpawnSystem {
    screen_width: u32,
    screen_height: u32,
    coin_timer: f32,
    enemy_timer: f32,
    powerup_timer: f32,
}

impl SpawnSystem {
    fn new(screen_width: u32, screen_height: u32) -> Self {
        SpawnSystem {
            screen_width,
            screen_height,
            coin_timer: 0.0,
            enemy_timer: 0.0,
            powerup_timer: 0.0,
        }
    }
}

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Coin>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, PowerUp>,
    );

    fn run(
        &mut self,
        (entities, mut positions, mut coins, mut enemies, mut powerups): Self::SystemData,
    ) {
        self.coin_timer += 0.1;
        self.enemy_timer += 0.1;
        self.powerup_timer += 0.1;

        let mut rng = rand::thread_rng();

        if self.coin_timer >= 5.0 {
            self.coin_timer = 0.0;
            let coin = entities.create();
            positions
                .insert(
                    coin,
                    Position {
                        x: rng.gen_range(0.0..self.screen_width as f32),
                        y: rng.gen_range(0.0..self.screen_height as f32),
                    },
                )
                .unwrap();
            coins.insert(coin, Coin).unwrap();
        }

        if self.enemy_timer >= 10.0 {
            self.enemy_timer = 0.0;
            let enemy = entities.create();
            positions
                .insert(
                    enemy,
                    Position {
                        x: rng.gen_range(0.0..self.screen_width as f32),
                        y: rng.gen_range(0.0..self.screen_height as f32),
                    },
                )
                .unwrap();
            enemies.insert(enemy, Enemy).unwrap();
        }

        if self.powerup_timer >= 15.0 {
            self.powerup_timer = 0.0;
            let powerup = entities.create();
            positions
                .insert(
                    powerup,
                    Position {
                        x: rng.gen_range(0.0..self.screen_width as f32),
                        y: rng.gen_range(0.0..self.screen_height as f32),
                    },
                )
                .unwrap();
            powerups
                .insert(
                    powerup,
                    PowerUp {
                        power_type: if rng.gen_bool(0.5) {
                            "health".to_string()
                        } else {
                            "speed".to_string()
                        },
                    },
                )
                .unwrap();
        }
    }
}

// --- Main Game Loop ---

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust ECS Game", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    // Initialize Specs World
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Health>();
    world.register::<Score>();
    world.register::<PlayerControlled>();
    world.register::<Coin>();
    world.register::<Enemy>();
    world.register::<Obstacle>();
    world.register::<PowerUp>();

    world.insert(InputResource::default()); // Insert an empty input resource

    // Create Entities
    let player = world
        .create_entity()
        .with(Position { x: 400.0, y: 300.0 })
        .with(Velocity { dx: 0.0, dy: 0.0 })
        .with(PlayerControlled)
        .with(Health { value: 100 })
        .with(Score { value: 0 })
        .build();

    let coin = world
        .create_entity()
        .with(Position { x: 200.0, y: 200.0 })
        .with(Coin)
        .build();

    let enemy = world
        .create_entity()
        .with(Position { x: 600.0, y: 100.0 })
        .with(Velocity { dx: 0.0, dy: 0.0 })
        .with(Enemy)
        .with(Health { value: 30 })
        .build();

    // Initialize Systems
    let mut render_system = RenderSystem {
        canvas: &mut canvas,
    };
    let mut player_input_system = PlayerInputSystem;
    let mut physics_system = PhysicsSystem;
    let mut spawn_system = SpawnSystem::new(800, 600);

    let mut running = true;
    while running {
        let keyboard_state = event_pump.keyboard_state();
        let keys: Vec<Scancode> = keyboard_state.pressed_scancodes().collect();
        world.write_resource::<InputResource>().keys = keys.iter().map(|k| Some(*k)).collect();

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
        // world.maintain();
    }

    Ok(())
}
