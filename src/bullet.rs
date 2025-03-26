use macroquad::math::Vec2;

use crate::target::Target;

pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
    pub rot: u32,
    pub team: Target,
}
