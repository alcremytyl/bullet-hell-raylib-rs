#![feature(portable_simd)]

use std::{
    process::exit,
    simd::f32x4,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use macroquad::{
    prelude::*,
    rand::{rand, srand},
};
use raylib_rs::{
    clamp, player::Player, traits::Drawable, Bullet, PLAYER_SIZE, PLAYER_SLOW, PLAYER_SPEED,
    PROJECTILE_CAP, SCREEN_H, SCREEN_W,
};

/*
TODO
- change player to a f32x4 such that [pos_x, pos_y, vel_x, vel_y]
- bullets (use simd somehow)
  - [x] spawning
  - [ ] despawning
  - [x] rendering
- enemies
-
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
// TODO:
// TODO:
// TODO:
// TODO:  add team enum
// TODO:
// TODO:

#[macroquad::main(conf)]
async fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // TODO: turn positions into f32x4
    let mut player = Player::new();
    let chunk_size: usize = 4;

    // pos_x, pos_y, vel_x, vel_y, lifespan, team
    let mut bullets = vec![vec![0.0; 0]; 5];

    srand(start.as_secs().into());

    loop {
        clear_background(WHITE);

        player.draw();
        draw_text(&format!("{bullets:?}"), 50.0, 50.0, 16.0, RED);

        // bullets
        if bullets[0].len() > 0 {
            let mut i = 0;

            while i + chunk_size < bullets[0].len() {
                let x = f32x4::from_slice(&bullets[0][i..chunk_size + i]);
                let y = f32x4::from_slice(&bullets[1][i..chunk_size + i]);
                let vx = f32x4::from_slice(&bullets[2][i..chunk_size + i]);
                let vy = f32x4::from_slice(&bullets[3][i..chunk_size + i]);

                // apply velocity
                bullets[0][i..chunk_size + i].copy_from_slice((x + vx).as_array());
                bullets[1][i..chunk_size + i].copy_from_slice((y + vy).as_array());

                i += chunk_size;
            }

            // apply to rest
            for i in i..bullets[0].len() {
                bullets[0][i] += bullets[2][i];
                bullets[1][i] += bullets[3][i];
            }

            for p in 0..bullets[0].len() {
                draw_circle(bullets[0][p], bullets[1][p], 30.0, BLUE);
            }
        }

        handle_input(&mut player.pos, &mut bullets);

        next_frame().await;
    }
}

fn handle_input(player_pos: &mut Vec2, bullets: &mut Vec<Vec<f32>>) {
    let displace = Vec2 {
        x: (is_key_down(KeyCode::A) as i32 - is_key_down(KeyCode::D) as i32) as f32,
        y: (is_key_down(KeyCode::W) as i32 - is_key_down(KeyCode::S) as i32) as f32,
    } * PLAYER_SPEED
        * (1.0 - (PLAYER_SLOW * is_key_down(KeyCode::LeftShift) as i32 as f32))
        * get_frame_time();

    player_pos.x = clamp(player_pos.x - displace.x, 0.0, SCREEN_W);
    player_pos.y = clamp(player_pos.y - displace.y, 0.0, SCREEN_H);

    // TODO: change to autofire
    if is_key_pressed(KeyCode::Space) {
        shoot(player_pos, bullets);
    }
    if is_key_down(KeyCode::Escape) {
        exit(0);
    }
}

fn shoot(pos: &Vec2, bullets: &mut Vec<Vec<f32>>) {
    println!("Magic Bullet #{} fired", bullets[0].len());
    bullets[0].push(pos.x);
    bullets[1].push(pos.y);
    bullets[2].push(0.0);
    bullets[3].push(-10.0);
    // bullets[4].push(Instant::now().as_secs() as f32);
    //
}
