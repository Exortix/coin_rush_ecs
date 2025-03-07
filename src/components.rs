use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Health {
    pub value: u32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Score {
    pub value: u32,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct PlayerControlled;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Coin;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Enemy;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Obstacle;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct PowerUp {
    pub power_type: String,
}

// Helper function to register all components
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Health>();
    world.register::<Score>();
    world.register::<PlayerControlled>();
    world.register::<Coin>();
    world.register::<Enemy>();
    world.register::<Obstacle>();
    world.register::<PowerUp>();
}
