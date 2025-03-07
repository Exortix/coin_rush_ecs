use sdl2::keyboard::Scancode;

#[derive(Default)]
pub struct InputResource {
    pub keys: Vec<Option<Scancode>>,
}

pub fn register_resources(world: &mut specs::World) {
    world.insert(InputResource::default());
}
