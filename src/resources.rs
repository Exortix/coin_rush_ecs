use sdl2::keyboard::Scancode;

#[derive(Default)]
pub struct InputResource {
    pub keys: Vec<Option<Scancode>>,
}
