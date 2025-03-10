use macroquad::{
    color::{Color, BLACK, WHITE},
    math::Vec2,
    shapes::draw_line,
    time::get_frame_time,
};

pub mod player;

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

/// pos_x, pos_y, vel_x, vel_y, lifespan, team
pub struct Bullets(pub Vec<Vec<f32>>);

impl Bullets {
    pub fn push(&mut self, bullet: Bullet, cooldown: &mut f32) {
        self.0[0].push(bullet.pos.x);
        self.0[1].push(bullet.pos.y);
        self.0[2].push(bullet.vel.x);
        self.0[3].push(-bullet.vel.y);
        self.0[4].push(bullet.lifespan);
        self.0[5].push(bullet.target as u32 as f32);

        *cooldown = get_frame_time() as f32 / FIRE_RATE;
    }

    pub fn draw() {}

    /// size of internal members, assumes all children are equal in length
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub enum Target {
    PLAYER = 1 << 0,
    FOE = 1 << 1,
}

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
