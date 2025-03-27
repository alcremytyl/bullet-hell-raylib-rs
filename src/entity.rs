use macroquad::math::Vec2;

use crate::{bullet::Bullet, target::Target};

pub struct EntityStats {
    pub angle: f32,
    pub health: u32,
    pub iframes: Option<f32>,
    pub iframe_duration: f32,
    pub pos: Vec2,
    pub target: Target,
}

pub trait Entity {
    // fn get_pos(&self) -> f32;
    // fn get_angle(&self) -> f32;
    //
    // fn get_mut_health(&mut self) -> &mut u32;
    // fn get_mut_iframes(&mut self) -> &mut Option<f32>;
    // fn get_iframe_duration(self) -> f32;
    // fn get_health(&self) -> u32;
    //
    // fn take_damage(&mut self, damage: u32) {
    //     *self.get_mut_health() -= damage;
    //     *self.get_mut_iframes() = *self.get_mut_iframes();
    // }

    fn shoot(&self) -> Vec<Bullet>;

    fn draw(&self);
}

// impl Drawable
