use macroquad::math::Vec2;

use crate::target::Target;

// TODO: custom textures
pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
    pub target: Target,
}

impl Bullet {
    pub fn new(pos: Vec2, vel: Vec2, size: Vec2, target: Target) -> Self {
        Bullet {
            pos,
            vel,
            size,
            target,
        }
    }
}
