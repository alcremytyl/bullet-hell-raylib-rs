use macroquad::prelude::Vec2;

use crate::weapon::Weapon;

pub struct Player {
    pub pos: Vec2,
    pub life: u8,
    pub weapon: Option<Weapon>,
    pub iframes: Option<f32>,
}
