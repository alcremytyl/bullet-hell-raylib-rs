use macroquad::{
    color::{BLACK, WHITE},
    input::{is_key_down, KeyCode},
    math::Vec2,
    text::draw_text,
    time::get_frame_time,
    window::{clear_background, screen_height, screen_width},
};

use crate::{
    bullet::Bullet,
    entity::{Entity, EntityStats},
    player::{Player, PLAYER_SLOW, PLAYER_SPEED},
    target::Target,
    weapon::Weapon,
};

pub struct Game {
    pub player: Player,
    pub bullets: Vec<Bullet>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: Player {
                stats: EntityStats {
                    pos: Vec2::new(screen_width() / 2.0, screen_height() / 8.0),
                    health: 3,
                    angle: 270.0,
                    iframes: None,
                    iframe_duration: 60.0 * 3.0,
                    target: Target::ENEMY,
                },
                weapon: Some(Weapon::PlayerBasic),
            },
            bullets: vec![],
        }
    }

    pub fn handle_input(&mut self) {
        let dt = get_frame_time();

        let displace = Vec2 {
            x: (is_key_down(KeyCode::A) as i32 - is_key_down(KeyCode::D) as i32) as f32,
            y: (is_key_down(KeyCode::W) as i32 - is_key_down(KeyCode::S) as i32) as f32,
        } * PLAYER_SPEED
            * dt
            * (1.0 - PLAYER_SLOW * is_key_down(KeyCode::LeftShift) as u32 as f32);

        self.player.stats.pos -= displace;
    }

    pub fn render(&self) {
        clear_background(WHITE);

        // draw player
        self.player.draw();

        // debug
        {
            let pos = self.player.stats.pos;
            let debug_txt = format!(
                "\
                    player pos: ({}, {})\n\
                    bullet count: N/A
                ",
                pos.x, pos.y
            );
            draw_text(&debug_txt, 20.0, 20.0, 12.0, BLACK);
        }
    }
}
