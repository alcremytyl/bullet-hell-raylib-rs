use macroquad::{
    color::{Color, BLACK, WHITE},
    math::Vec2,
    shapes::draw_line,
};

pub mod player;
pub mod traits;

pub const PROJECTILE_CAP: usize = 500;
pub const PLAYER_SIZE: f32 = 12.0; // assume half
pub const PLAYER_SPEED: f32 = 250.0;
pub const PLAYER_SLOW: f32 = 0.6;
pub const SCREEN_W: f32 = 1280.0;
pub const SCREEN_H: f32 = 720.0;
pub const BG_COLOR: Color = WHITE;

pub struct Bullet {
    team: Team,
    pos: Vec2,
    velocity: Vec2,
}

impl Bullet {
    pub fn new(team: Team, pos: Vec2, velocity: Vec2) -> Self {
        Bullet {
            team,
            pos,
            velocity,
        }
    }
}

pub enum Team {
    PLAYER,
    FOE,
}

pub fn clamp<T: PartialOrd>(n: T, lowest: T, highest: T) -> T {
    let tmp = if n < lowest { lowest } else { n };
    return if tmp > highest { highest } else { tmp };
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
            draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, BLACK);
            p2
        })
        .unwrap();
}
