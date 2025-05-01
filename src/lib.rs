use macroquad::{color::Color, math::Vec2, shapes::draw_line};

pub mod bullet;
pub mod entity;
pub mod game;
pub mod player;
pub mod target;
pub mod weapon;

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

pub trait Draw {
    fn draw(&self);
}
