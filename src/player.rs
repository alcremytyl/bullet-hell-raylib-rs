use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{drawable::Drawable, PLAYER_SIZE, SCREEN_H, SCREEN_W};

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
        let displace = Vector2 {
            x: 0.0,
            y: PLAYER_SIZE,
        };
        handler.draw_line_ex(self.pos - displace, self.pos + displace, 3.0, Color::BLACK);
    }
}
