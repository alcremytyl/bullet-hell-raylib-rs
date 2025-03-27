use macroquad::{color::BLACK, math::Vec2, shapes::draw_triangle_lines};

use crate::{
    bullet::Bullet,
    draw_outline,
    entity::{Entity, EntityStats},
    weapon::Weapon,
};

pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SLOW: f32 = 0.6;
pub const PLAYER_SIZE: f32 = 12.0;
const SCALE: f32 = 2.5;

pub struct Player {
    pub stats: EntityStats,
    pub weapon: Option<Weapon>,
}

impl Entity for Player {
    fn shoot(&self) -> Vec<Bullet> {
        if let Some(w) = &self.weapon {
            return w.shoot(&self.stats.pos, self.stats.angle);
        }

        // TODO: graphical version
        println!("player has no weapon");
        return vec![];
    }

    fn draw(&self) {
        let p = self.stats.pos;
        let v = Vec2::new(0.0, PLAYER_SIZE);
        let h = Vec2::new(PLAYER_SIZE, 0.0);

        let coords = [
            Vec2::new(p.x - h.x, p.y + v.y),
            Vec2::new(p.x + h.x, p.y + v.y),
            Vec2::new(p.x, p.y - v.y),
            Vec2::new(p.x - h.x, p.y + v.y),
        ];

        draw_triangle_lines(coords[0], coords[1], coords[2], 3.0, BLACK);
        draw_outline(&coords, SCALE, BLACK);
    }
}
