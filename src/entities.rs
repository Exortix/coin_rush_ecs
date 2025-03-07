use crate::components::{
    BoundingBox, Coin, Collidable, Enemy, Health, PlayerControlled, Position, Score, Velocity,
};
use specs::prelude::*;

pub fn create_player(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Position { x: 400.0, y: 300.0 })
        .with(Velocity { dx: 0.0, dy: 0.0 })
        .with(BoundingBox {
            width: 20.0,
            height: 20.0,
        })
        .with(PlayerControlled)
        .with(Collidable)
        .with(Health { value: 100 })
        .with(Score { value: 0 })
        .build()
}

pub fn create_initial_coin(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Position { x: 200.0, y: 200.0 })
        .with(BoundingBox {
            width: 10.0,
            height: 10.0,
        })
        .with(Coin)
        .with(Collidable)
        .build()
}

pub fn create_initial_enemy(world: &mut World) -> Entity {
    world
        .create_entity()
        .with(Position { x: 600.0, y: 100.0 })
        .with(Velocity { dx: 0.0, dy: 0.0 })
        .with(BoundingBox {
            width: 20.0,
            height: 20.0,
        })
        .with(Enemy)
        .with(Collidable)
        .with(Health { value: 30 })
        .build()
}
