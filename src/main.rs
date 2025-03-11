#![feature(portable_simd)]
use std::{
    cell::RefCell,
    process::exit,
    simd::{cmp::SimdPartialOrd, f32x64},
    sync::{LazyLock, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use macroquad::{
    prelude::*,
    rand::{rand, srand},
};
use raylib_rs::{
    player::Player, traits::Drawable, Bullet, Bullets, Target, BULLET_DEVIATION, CHUNK_SIZE,
    FIRE_RATE, PLAYER_SLOW, PLAYER_SPEED, SCREEN_H, SCREEN_W,
};

/*
TODO
- bullet coll
- enemies
- scale all movement to screen size
- everything
*/

// NOTE: testing

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
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut player = Player::new();
    let mut cooldown = 0f32;

    let mut bullets = Bullets(vec![vec![0.0; 500]; 6]);
    let zeroes = f32x64::splat(0.0);

    bullets.shoot(
        Bullet::new(
            Vec2::new(SCREEN_W / 2.0, SCREEN_H / 2.0 + 100.0),
            Vec2::new(0.0, 0.0),
            1000.0,
            Target::PLAYER,
        ),
        &mut cooldown,
    );

    srand(start.as_secs().into());

    loop {
        let dt = get_frame_time();

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
            for i in i..bullets.len() {
                bullets.0[0][i] += bullets.0[2][i] * dt;
                bullets.0[1][i] += bullets.0[3][i] * dt;
                bullets.0[4][i] -= dt;
                to_delete[i] = bullets.0[4][i] <= 0.0;
            }

            // NOTE: may change if the optimization brain worms win
            for i in (0..to_delete.len()).rev() {
                if to_delete[i] {
                    for j in 0..bullets.len() {
                        bullets[j].remove(i);
                    }
                }
            }
        }

        clear_background(WHITE);

        bullets.draw();

        for j in 0..bullets.len() {
            draw_circle(bullets[0][j], bullets[1][j], 10.0, BLUE);
        }

        cooldown = clamp(cooldown - dt, 0.0, FIRE_RATE / get_fps() as f32);

        player.draw();
        draw_text(&format!("{:?}", bullets.0[0].len()), 50.0, 50.0, 16.0, RED);
        draw_text(&format!("{cooldown}"), 50.0, 150.0, 16.0, RED);

        handle_input(&mut player.pos, &mut bullets, &mut cooldown);
        next_frame().await;
    }
}

fn handle_input(player_pos: &mut Vec2, bullets: &mut Bullets, cooldown: &mut f32) {
    let displace = Vec2 {
        x: (is_key_down(KeyCode::A) as i32 - is_key_down(KeyCode::D) as i32) as f32,
        y: (is_key_down(KeyCode::W) as i32 - is_key_down(KeyCode::S) as i32) as f32,
    } * PLAYER_SPEED
        * (1.0 - (PLAYER_SLOW * is_key_down(KeyCode::LeftShift) as i32 as f32))
        * get_frame_time();

    player_pos.x = (player_pos.x - displace.x).clamp(0.0, screen_width());
    player_pos.y = (player_pos.y - displace.y).clamp(0.0, screen_height());

    if is_key_down(KeyCode::Space) && *cooldown == 0.0 {
        let b = Bullet::new(
            *player_pos,
            Vec2::new(
                (rand() as f32 % BULLET_DEVIATION) - BULLET_DEVIATION / 2.0,
                1500.0,
            ),
            1.0,
            Target::FOE,
        );

        bullets.push(b, cooldown);
    }
    if is_key_down(KeyCode::Escape) {
        exit(0);
    }
}

// TODO: determine some direction-based deviation
