use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};

use crate::{bullet::Bullet, player::Player, weapon::Weapon};

pub struct Game {
    pub player: Player,
    pub bullets: Vec<Bullet>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: Player {
                pos: Vec2::new(screen_width() / 2.0, screen_height() / 8.0),
                life: 3,
                weapon: todo!(),
                iframes: None,
            },
            bullets: vec![],
        }
    }
}
