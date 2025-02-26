pub mod drawable;
pub mod player;

use raylib::math::Vector2;

pub const PROJECTILE_CAP: u16 = 500;
pub const PLAYER_SIZE: f32 = 16.0; // assume half
pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SLOW: f32 = 0.6;
pub const SCREEN_W: f32 = 1280.0;
pub const SCREEN_H: f32 = 720.0;

pub struct Bullet {
    team: Team,
    pos: Vector2,
    velocity: Vector2,
}

pub enum Team {
    PLAYER,
    FOE,
}

pub fn clamp<T: PartialOrd>(n: T, lowest: T, highest: T) -> T {
    let tmp = if n < lowest { lowest } else { n };
    return if tmp > highest { highest } else { tmp };
}
