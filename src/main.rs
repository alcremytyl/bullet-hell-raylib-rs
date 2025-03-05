#![feature(portable_simd)]

use std::{
    process::exit,
    simd::{cmp::SimdPartialOrd, f32x4, f32x64},
    time::{Duration, SystemTime, UNIX_EPOCH},
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
- bullet coll
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
const FIRE_RATE: f32 = 1000.0;
const BULLET_DEVIATION: f32 = 1000.0;
const CHUNK_SIZE: usize = 64;

#[macroquad::main(conf)]
async fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut player = Player::new();
    let mut cooldown = 0f32;

    let mut bullets: Bullets = vec![vec![0.0; 0]; 6];
    let zeroes = f32x64::splat(0.0);

    srand(start.as_secs().into());

    loop {
        let dt = get_frame_time();

        clear_background(WHITE);

        player.draw();
        draw_text(&format!("{:?}", bullets[0].len()), 50.0, 50.0, 16.0, RED);

        // bullets
        if bullets[0].len() > 0 {
            let mut i = 0;
            let mut to_delete = vec![false; bullets[0].len()];
            let dt_reg = f32x64::splat(dt);

            while i + CHUNK_SIZE < bullets[0].len() + 1 {
                let j = i..CHUNK_SIZE + i;

                let x = f32x64::from_slice(&bullets[0][j.clone()]);
                let y = f32x64::from_slice(&bullets[1][j.clone()]);
                let vx = f32x64::from_slice(&bullets[2][j.clone()]);
                let vy = f32x64::from_slice(&bullets[3][j.clone()]);
                let lf = f32x64::from_slice(&bullets[4][j.clone()]);
                // let tm = f32x64::from_slice(&bullets[5][j.clone()]);

                // update vectors
                bullets[0][j.clone()].copy_from_slice((x + dt_reg * vx).as_array());
                bullets[1][j.clone()].copy_from_slice((y + dt_reg * vy).as_array());
                bullets[4][j.clone()].copy_from_slice((lf - dt_reg).as_array());
                to_delete[j.clone()].copy_from_slice(&lf.simd_le(zeroes).to_array());

                i += CHUNK_SIZE;
            }

            // apply to rest
            for i in i..bullets[0].len() {
                bullets[0][i] += bullets[2][i] * dt;
                bullets[1][i] += bullets[3][i] * dt;
                bullets[4][i] -= dt;
                to_delete[i] = bullets[4][i] <= 0.0;
            }

            for i in (0..to_delete.len()).rev() {
                println!("checking {i}");
                if to_delete[i] {
                    for j in 0..bullets.len() {
                        bullets[j].remove(i);
                    }
                }
            }

            draw_text(&format!("{to_delete:?}"), 50.0, 100.0, 16.0, RED);

            for j in 0..bullets[0].len() {
                draw_circle(bullets[0][j], bullets[1][j], 10.0, BLUE);
            }
        }

        cooldown = clamp(cooldown - dt, 0.0, FIRE_RATE / get_fps() as f32);
        draw_text(&format!("{cooldown}"), 50.0, 150.0, 16.0, RED);

        handle_input(&mut player.pos, &mut bullets, &start, &mut cooldown);
        next_frame().await;
    }
}

fn handle_input(
    player_pos: &mut Vec2,
    bullets: &mut Vec<Vec<f32>>,
    start: &Duration,
    cooldown: &mut f32,
) {
    let displace = Vec2 {
        x: (is_key_down(KeyCode::A) as i32 - is_key_down(KeyCode::D) as i32) as f32,
        y: (is_key_down(KeyCode::W) as i32 - is_key_down(KeyCode::S) as i32) as f32,
    } * PLAYER_SPEED
        * (1.0 - (PLAYER_SLOW * is_key_down(KeyCode::LeftShift) as i32 as f32))
        * get_frame_time();

    player_pos.x = (player_pos.x - displace.x).clamp(0.0, SCREEN_W);
    player_pos.y = (player_pos.y - displace.y).clamp(0.0, SCREEN_H);

    // TODO: change to autofire
    if is_key_down(KeyCode::Space) && *cooldown == 0.0 {
        shoot(player_pos, bullets, 1.0, Targets::FOE, cooldown);
    }
    if is_key_down(KeyCode::Escape) {
        exit(0);
    }
}

fn shoot(
    pos: &Vec2,
    bullets: &mut Vec<Vec<f32>>,
    lifespan: f64,
    targets: Targets,
    cooldown: &mut f32,
) {
    bullets[0].push(pos.x);
    bullets[1].push(pos.y);
    bullets[2].push((rand() as f32 % BULLET_DEVIATION) - BULLET_DEVIATION / 2.0);
    bullets[3].push(-1500.0);
    bullets[4].push(lifespan as f32);
    bullets[5].push(Targets::FOE as u32 as f32);

    bullets[0].push(pos.x);
    bullets[1].push(pos.y);
    bullets[2].push((rand() as f32 % BULLET_DEVIATION) - BULLET_DEVIATION / 2.0);
    bullets[3].push(-1500.0);
    bullets[4].push(lifespan as f32);
    bullets[5].push(targets as u32 as f32);

    *cooldown = get_frame_time() as f32 / FIRE_RATE;
}
