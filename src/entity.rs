use macroquad::math::Vec2;

use crate::{bullet::Bullet, target::Target, weapon::Weapon, Draw};

pub struct Entity {
    pub angle: f32,
    pub health: u32,
    pub iframe_duration: f32,
    pub iframes: Option<f32>,
    pub pos: Vec2,
    pub target: Option<Target>,
    pub vel: Vec2,
    pub weapon: Option<Weapon>,
}

impl Entity {
    fn take_damage(&mut self, damage: u32) {
        *self.health -= damage;
        *self.iframes = *self.iframe_duration;
    }

    fn shoot(&self) -> Vec<Bullet> {
        match self.weapon {
            Some(w) => w.shoot(&self.pos, self.angle),
            None => {
                println!("no weapon");
                vec![]
            }
        }
    }

    fn update(&self) {
        unimplemented!()
    }
}

impl Draw for Entity {
    fn draw(&self) {
        unimplemented!("Implement drawing");
    }
}
