use crate::components::{BoundingBox, Coin, Collidable, Enemy, Position, PowerUp};
use rand::Rng;
use specs::*;

pub struct SpawnSystem {
    screen_width: u32,
    screen_height: u32,
    coin_timer: f32,
    enemy_timer: f32,
    powerup_timer: f32,
}

impl SpawnSystem {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
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
        WriteStorage<'a, BoundingBox>,
        WriteStorage<'a, Collidable>,
        WriteStorage<'a, Coin>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, PowerUp>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut positions,
            mut bounding_boxes,
            mut collidables,
            mut coins,
            mut enemies,
            mut powerups,
        ): Self::SystemData,
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
            bounding_boxes
                .insert(
                    coin,
                    BoundingBox {
                        width: 10.0,
                        height: 10.0,
                    },
                )
                .unwrap();
            collidables.insert(coin, Collidable).unwrap();
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
            bounding_boxes
                .insert(
                    enemy,
                    BoundingBox {
                        width: 20.0,
                        height: 20.0,
                    },
                )
                .unwrap();
            collidables.insert(enemy, Collidable).unwrap();
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
            bounding_boxes
                .insert(
                    powerup,
                    BoundingBox {
                        width: 10.0,
                        height: 10.0,
                    },
                )
                .unwrap();
            collidables.insert(powerup, Collidable).unwrap();
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
