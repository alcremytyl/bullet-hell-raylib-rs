use macroquad::{color::BLACK, math::Vec2, shapes::draw_triangle_lines};

use crate::{
    draw_outline,
    traits::{Drawable, Entity},
    PLAYER_SIZE, SCREEN_H, SCREEN_W,
};

pub struct Player {
    pub pos: Vec2,
    pub fire_rate: i32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vec2::new(SCREEN_W / 2.0, SCREEN_H / 2.0),
            fire_rate: 10,
        }
    }
}

impl Drawable for Player {
    fn draw(&self) {
        const SCALE: f32 = 2.5;
        let v = Vec2::new(0.0, PLAYER_SIZE);
        let h = Vec2::new(PLAYER_SIZE, 0.0);

        let coords = [
            Vec2::new(self.pos.x - h.x, self.pos.y + v.y),
            Vec2::new(self.pos.x + h.x, self.pos.y + v.y),
            Vec2::new(self.pos.x, self.pos.y - v.y),
            Vec2::new(self.pos.x - h.x, self.pos.y + v.y),
        ];

        draw_triangle_lines(coords[0], coords[1], coords[2], 3.0, BLACK);
        draw_outline(&coords, SCALE, BLACK);
    }
}

impl Entity for Player {}
