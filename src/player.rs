use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{draw_outline, drawable::Drawable, PLAYER_SIZE, SCREEN_H, SCREEN_W};

pub struct Player {
    pub pos: Vector2,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector2 {
                x: SCREEN_W / 2.0,
                y: SCREEN_H / 2.0,
            },
        }
    }
}

impl Drawable for Player {
    fn draw(&self, handler: &mut RaylibDrawHandle) {
        const SCALE: f32 = 2.5;
        let v = Vector2::new(0.0, PLAYER_SIZE);
        let h = Vector2::new(PLAYER_SIZE, 0.0);

        let coords = [
            Vector2::new(self.pos.x - h.x, self.pos.y + v.y),
            Vector2::new(self.pos.x + h.x, self.pos.y + v.y),
            Vector2::new(self.pos.x, self.pos.y - v.y),
            Vector2::new(self.pos.x - h.x, self.pos.y + v.y),
        ];

        handler.draw_triangle_lines(&coords[0], &coords[1], &coords[2], Color::BLACK);
        draw_outline(handler, &coords, SCALE, Color::BLACK);
    }
}
