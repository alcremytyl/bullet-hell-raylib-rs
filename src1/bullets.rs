use std::ops::{Index, IndexMut};

use macroquad::{shapes::draw_circle, time::get_frame_time};

use crate::{target::Target, Bullet, FIRE_RATE};

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
        self[POS_X].push(bullet.pos.x);
        self[POS_Y].push(bullet.pos.y);
        self[VEL_X].push(bullet.vel.x);
        self[VEL_Y].push(-bullet.vel.y);
        self[LIFESPAN].push(bullet.lifespan);
        self[TARGET].push(bullet.target.into());

        *cooldown = get_frame_time() as f32 / FIRE_RATE;
    }

    pub fn draw(&self) {
        println!("outer {} {}", self.len(), self.0[TARGET].len());
        for i in 0..self.len() {
            println!("{i} {}", self.0[TARGET].len());
            let color = Target::from(self.0[TARGET][i]).as_color();

            draw_circle(self.0[POS_X][i], self.0[POS_Y][i], 10.0, color);
        }
    }

    /// size of internal members, assumes all children are equal in length
    pub fn len(&self) -> usize {
        self.0[0].len()
    }
}

impl Index<usize> for Bullets {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Bullets {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
