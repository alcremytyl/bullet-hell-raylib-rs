use macroquad::math::Vec2;

use crate::target::Target;

pub struct EntityStats {
    health: u32,
    pos: Vec2,
    angle: u32,
    target: Target,
}

pub trait Entity {
    fn get_pos(&self) -> f32;
    fn get_angle(&self) -> f32;
}
