#![feature(portable_simd)]

use std::{
    process::exit,
    simd::{cmp::SimdPartialOrd, f32x4, LaneCount, Mask},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use macroquad::{
    prelude::*,
    rand::{rand, srand},
};
use raylib_rs::{
    player::Player, traits::Drawable, Targets, PLAYER_SIZE, PLAYER_SLOW, PLAYER_SPEED,
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

/// pos_x, pos_y, vel_x, vel_y, lifespan, team
type Bullets = Vec<Vec<f32>>;

#[macroquad::main(conf)]
async fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // TODO: turn positions into f32x4
    let mut player = Player::new();
    const CHUNK_SIZE: usize = 4;

    let mut bullets: Bullets = vec![vec![0.0; 0]; 6];
    let zeroes = f32x4::splat(0.0);

    srand(start.as_secs().into());

    loop {
        clear_background(WHITE);

        player.draw();
        draw_text(&format!("{:.1?}", bullets[4]), 50.0, 50.0, 16.0, RED);

        // bullets
        if bullets[0].len() > 0 {
            let mut i = 0;
            let j = i..CHUNK_SIZE + i;
            let dt = get_frame_time();
            let mut to_delete = vec![false; bullets[0].len()];

            // FIXMEL crashes when n >= 5
            while i + CHUNK_SIZE < bullets[0].len() {
                let x = f32x4::from_slice(&bullets[0][j.clone()]);
                let y = f32x4::from_slice(&bullets[1][j.clone()]);
                let vx = f32x4::from_slice(&bullets[2][j.clone()]);
                let vy = f32x4::from_slice(&bullets[3][j.clone()]);
                let lf = f32x4::from_slice(&bullets[4][j.clone()]);
                let tm = f32x4::from_slice(&bullets[5][j.clone()]);

                // update vectors
                bullets[0][j.clone()].copy_from_slice((x + vx).as_array());
                bullets[1][j.clone()].copy_from_slice((y + vy).as_array());
                bullets[4][j.clone()].copy_from_slice((lf - f32x4::splat(dt)).as_array());
                // FIXME: investigate here
                to_delete[j.clone()].copy_from_slice(&lf.simd_le(zeroes).to_array());

                i += CHUNK_SIZE;
            }

            // apply to rest
            for i in i..bullets[0].len() {
                bullets[0][i] += bullets[2][i];
                bullets[1][i] += bullets[3][i];
                bullets[4][i] -= dt;
                to_delete[i] = bullets[4][i] <= 0.0;
            }

            // FIXME: investigate here
            for i in i..bullets[0].len() {
                if to_delete[i] {
                    for j in 0..bullets.len() {
                        println!("removed");
                        bullets[j].remove(i);
                    }
                }
            }

            draw_text(&format!("{to_delete:?}"), 50.0, 100.0, 16.0, RED);

            for j in 0..bullets[0].len() {
                draw_circle(bullets[0][j], bullets[1][j], 10.0, BLUE);
            }
        }

        handle_input(&mut player.pos, &mut bullets, &start);

        next_frame().await;
    }
}

fn handle_input(player_pos: &mut Vec2, bullets: &mut Vec<Vec<f32>>, start: &Duration) {
    let displace = Vec2 {
        x: (is_key_down(KeyCode::A) as i32 - is_key_down(KeyCode::D) as i32) as f32,
        y: (is_key_down(KeyCode::W) as i32 - is_key_down(KeyCode::S) as i32) as f32,
    } * PLAYER_SPEED
        * (1.0 - (PLAYER_SLOW * is_key_down(KeyCode::LeftShift) as i32 as f32))
        * get_frame_time();

    player_pos.x = (player_pos.x - displace.x).clamp(0.0, SCREEN_W);
    player_pos.y = (player_pos.y - displace.y).clamp(0.0, SCREEN_H);

    // TODO: change to autofire
    if is_key_pressed(KeyCode::Space) {
        shoot(player_pos, bullets, 1.0, Targets::FOE);
    }
    if is_key_down(KeyCode::Escape) {
        exit(0);
    }
}

fn shoot(pos: &Vec2, bullets: &mut Vec<Vec<f32>>, lifespan: f64, targets: Targets) {
    println!("Magic Bullet #{} fired", bullets[0].len());
    bullets[0].push(pos.x);
    bullets[1].push(pos.y);
    bullets[2].push(0.0);
    bullets[3].push(-10.0);
    bullets[4].push(lifespan as f32);
    bullets[5].push(targets as u32 as f32);
}
