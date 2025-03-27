use macroquad::math::Vec2;

use crate::{bullet::Bullet, target::Target};

pub enum Weapon {
    PlayerBasic,
    PlayerDouble,
}

impl Weapon {
    pub fn shoot(&self, pos: &Vec2, angle: f32) -> Vec<Bullet> {
        match self {
            Self::PlayerBasic => {
                vec![Bullet::new(
                    *pos,
                    Vec2::new(0.0, 200.0) * Vec2::from_angle(angle),
                    Vec2::new(10.0, 10.0),
                    Target::ENEMY,
                )]
            }
            Self::PlayerDouble => {
                vec![]
            }
        }
    }
}
