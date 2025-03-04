#![feature(portable_simd)]

use std::simd::{f32x8, StdFloat};

use macroquad::prelude::*;
use raylib_rs::{
    clamp, player::Player, traits::Drawable, Bullet, PLAYER_SIZE, PLAYER_SLOW, PLAYER_SPEED,
    PROJECTILE_CAP, SCREEN_H, SCREEN_W,
};

/*
TODO
- bullets (use simd somehow)
- enemies
- everything
*/

fn conf() -> Conf {
    Conf {
        window_title: "Bullet Hell".into(),
        window_height: SCREEN_H as i32,
        window_width: SCREEN_W as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut player = Player::new();

    let mut time = 0.0;

    let mut test_particles_x = vec![0.0; 20];
    let mut test_particles_y = (0..20).map(|i| 50.0 * i as f32).collect::<Vec<f32>>();

    loop {
        clear_background(WHITE);
        // TODO: draw fps

        player.draw();

        let chunk_size = 8;
        let mut i = 0;

        while i + chunk_size <= 20 {
            let x = f32x8::from_slice(&test_particles_x[i..chunk_size + i]);
            let new_x = x + f32x8::splat(2.0);
            let sine = (x + f32x8::splat(time)).sin();

            test_particles_x[i..chunk_size + i].copy_from_slice(&new_x.to_array());
            test_particles_y[i..chunk_size + i].copy_from_slice(&sine.to_array());

            i += chunk_size;
        }

        // stragglers
        while i < 20 {
            test_particles_x[i] += 2.0;
            test_particles_y[i] = (test_particles_x[i] + time).sin();
            i += 1
        }

        for p in 0..20 {
            draw_circle(test_particles_x[p], test_particles_y[p], 30.0, PINK);
        }

        handle_input(&mut player.pos);
        time += 0.2;

        next_frame().await;
    }
}

fn handle_input(player_pos: &mut Vec2) {
    let displace = Vec2 {
        x: (is_key_down(KeyCode::A) as i32 - is_key_down(KeyCode::D) as i32) as f32,
        y: (is_key_down(KeyCode::W) as i32 - is_key_down(KeyCode::S) as i32) as f32,
    } * PLAYER_SPEED
        * (1.0 - (PLAYER_SLOW * is_key_down(KeyCode::LeftShift) as i32 as f32))
        * get_frame_time();

    player_pos.x = clamp(player_pos.x - displace.x, 0.0, SCREEN_W);
    player_pos.y = clamp(player_pos.y - displace.y, 0.0, SCREEN_H);

    if is_key_down(KeyCode::Space) {}
}

fn shoot() {}
