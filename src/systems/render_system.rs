use crate::components::{Coin, Enemy, PlayerControlled, Position};
use sdl2::render::Canvas;
use sdl2::video::Window;
use specs::prelude::*;

pub struct RenderSystem<'a> {
    pub canvas: &'a mut Canvas<Window>,
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, PlayerControlled>,
        ReadStorage<'a, Coin>,
        ReadStorage<'a, Enemy>,
    );

    fn run(&mut self, (positions, players, coins, enemies): Self::SystemData) {
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Render Players
        for (pos, _) in (&positions, &players).join() {
            self.canvas
                .set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
            self.canvas
                .fill_rect(sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, 20, 20))
                .unwrap();
        }

        // Render Coins
        for (pos, _) in (&positions, &coins).join() {
            self.canvas
                .set_draw_color(sdl2::pixels::Color::RGB(255, 255, 0));
            self.canvas
                .fill_rect(sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, 10, 10))
                .unwrap();
        }

        // Render Enemies
        for (pos, _) in (&positions, &enemies).join() {
            self.canvas
                .set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
            self.canvas
                .fill_rect(sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, 20, 20))
                .unwrap();
        }

        self.canvas.present();
    }
}
