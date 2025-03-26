use std::{default, env::Args};

use macroquad::math::Vec2;

use crate::{
    bullet::Bullet,
    entity::{Entity, EntityStats},
    target::Target,
};

pub struct Weapon<'a> {
    pub cooldown: Option<f32>,
    pub shoot: Box<dyn FnMut() -> Bullet>,
    pub x: f32,
    pub parent: &'a EntityStats,
}
