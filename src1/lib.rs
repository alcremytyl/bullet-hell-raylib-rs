use macroquad::{
    color::{Color, WHITE},
    math::Vec2,
    shapes::draw_line,
};
use target::Target;

pub mod bullets;
pub mod player;
pub mod target;

pub mod traits {
    pub trait Drawable {
        fn draw(&self);
    }

    pub trait Entity {}
}

pub const BG_COLOR: Color = WHITE;
pub const BULLET_DEVIATION: f32 = 1000.0;
pub const CHUNK_SIZE: usize = 64;
pub const FIRE_RATE: f32 = 1000.0;
pub const PLAYER_SIZE: f32 = 12.0; // assume half
pub const PLAYER_SLOW: f32 = 0.6;
pub const PLAYER_SPEED: f32 = 250.0;
pub const SCREEN_H: f32 = 720.0;
pub const SCREEN_W: f32 = 1280.0;

pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub lifespan: f32,
    pub target: Target,
}

impl Bullet {
    pub fn new(pos: Vec2, vel: Vec2, lifespan: f32, target: Target) -> Self {
        Self {
            pos,
            vel,
            lifespan,
            target,
        }
    }
}

pub fn draw_outline(points: &[Vec2], scale: f32, color: Color) {
    let size = points.len() - 1; // account for last point going back to origin

    let center = (0..size)
        .map(|i| points[i])
        .reduce(|v1, v2| v1 + v2)
        .unwrap()
        / size as f32;

    let prime: Vec<Vec2> = (0..points.len())
        .map(|i| {
            Vec2 {
                x: -(points[i].x - center.x),
                y: points[i].y - center.y,
            } * scale
                + center
        })
        .collect();

    prime
        .iter()
        .reduce(|p1, p2| {
            draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, color);
            p2
        })
        .unwrap();
}
