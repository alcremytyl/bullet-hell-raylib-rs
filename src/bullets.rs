use macroquad::{shapes::draw_circle, time::get_frame_time};

use crate::{Bullet, FIRE_RATE};

pub const POS_X: usize = 0;
pub const POS_Y: usize = 1;
pub const VEL_X: usize = 2;
pub const VEL_Y: usize = 3;
pub const LIFESPAN: usize = 4;
pub const TARGET: usize = 5;

/// pos_x, pos_y, vel_x, vel_y, lifespan, target (enum as f32)
pub struct Bullets(pub Vec<Vec<f32>>);

impl Bullets {
    pub fn push(&mut self, bullet: Bullet, cooldown: &mut f32) {
        self.0[0].push(bullet.pos.x);
        self.0[1].push(bullet.pos.y);
        self.0[2].push(bullet.vel.x);
        self.0[3].push(-bullet.vel.y);
        self.0[4].push(bullet.lifespan);
        self.0[5].push(bullet.target.into());

        *cooldown = get_frame_time() as f32 / FIRE_RATE;
    }

    pub fn draw(&self) {
        for i in 0..self.len() {
            let color = draw_circle(self.0[0][i], self.0[1][i], 10.0, color);
        }
    }

    /// size of internal members, assumes all children are equal in length
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
