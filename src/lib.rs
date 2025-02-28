pub mod drawable;
pub mod player;

use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

pub const PROJECTILE_CAP: u16 = 500;
pub const PLAYER_SIZE: f32 = 12.0; // assume half
pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SLOW: f32 = 0.6;
pub const SCREEN_W: f32 = 1280.0;
pub const SCREEN_H: f32 = 720.0;
pub const BG_COLOR: Color = Color::RAYWHITE;

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

pub fn draw_outline(handler: &mut RaylibDrawHandle, points: &[Vector2], scale: f32, color: Color) {
    let size = points.len() - 1; // account for last point going back to origin

    let center = (0..size)
        .map(|i| points[i])
        .reduce(|v1, v2| v1 + v2)
        .unwrap()
        / size as f32;

    let prime: Vec<Vector2> = (0..points.len())
        .map(|i| {
            Vector2 {
                x: -(points[i].x - center.x),
                y: points[i].y - center.y,
            } * scale
                + center
        })
        .collect();

    handler.draw_line_strip(&prime, color);
}
