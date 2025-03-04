use macroquad::prelude::*;
use miniquad::conf::Platform;
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
        platform: Platform {
            swap_interval: Some(60),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut player = Player::new();

    loop {
        clear_background(WHITE);
        // TODO: draw fps
        draw_text(format!("a"), x, y, font_size, color);

        player.draw();
        handle_input(&mut player.pos);

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
